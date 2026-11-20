package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.entity.animal.greatwhiteshark.GreatWhiteShark;
import com.google.gson.Gson;
import com.mojang.authlib.GameProfile;
import java.util.LinkedHashMap;
import java.util.UUID;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.network.Connection;
import net.minecraft.network.chat.Component;
import net.minecraft.network.protocol.PacketFlow;
import net.minecraft.server.level.ClientInformation;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.server.network.CommonListenerCookie;
import net.minecraft.server.network.ServerGamePacketListenerImpl;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityTypes;
import net.minecraft.world.entity.Mob;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.entity.ai.goal.GoalSelector;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.minecraft.world.level.GameType;
import net.minecraft.world.level.block.Blocks;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class SharkKillPlayerProbe {
    public SharkKillPlayerProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("shark_kill_player").executes(ctx -> {
            var level = ctx.getSource().getLevel();
            var shark = create(level, new BlockPos(8, 92, 6));
            var player = player(level);
            player.setHealth(1.0F);
            var values = new LinkedHashMap<String, Object>();
            values.put("hit", shark.doHurtTarget(level, player));
            values.put("player_dead", !player.isAlive());
            values.put("cooldown", shark.getHuntingCooldown());
            shark.aiStep();
            values.put("cooldown_next_tick", shark.getHuntingCooldown());
            shark.discard();
            player.discard();
            ctx.getSource().sendSuccess(() -> Component.literal("shark_kill_player " + new Gson().toJson(values)), false);
            return 1;
        }));
    }

    private static GreatWhiteShark create(ServerLevel level, BlockPos pos) {
        var shark = NaturalistEntityTypes.GREAT_WHITE_SHARK.get().create(level, EntitySpawnReason.COMMAND);
        shark.snapTo(pos.getX() + 0.5, pos.getY(), pos.getZ() + 0.5, 0, 0);
        shark.setNoAi(true);
        shark.setNoGravity(true);
        level.addFreshEntity(shark);
        return shark;
    }

    private static ServerPlayer player(ServerLevel level) {
        var profile = new GameProfile(UUID.randomUUID(), "SharkFixture");
        var player = new ServerPlayer(level.getServer(), level, profile, ClientInformation.createDefault());
        player.connection = new ServerGamePacketListenerImpl(level.getServer(), new Connection(PacketFlow.SERVERBOUND), player,
            CommonListenerCookie.createInitial(profile, false));
        player.snapTo(10.5, 92, 6.5, 0, 0);
        level.addNewPlayer(player);
        player.connection.markClientLoaded();
        player.setGameMode(GameType.SURVIVAL);
        return player;
    }
}
