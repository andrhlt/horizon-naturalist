package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.google.gson.Gson;
import com.mojang.authlib.GameProfile;
import java.util.LinkedHashMap;
import java.util.List;
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
import net.minecraft.world.entity.Entity;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.Mob;
import net.minecraft.world.entity.ai.goal.GoalSelector;
import net.minecraft.world.entity.ai.goal.WrappedGoal;
import net.minecraft.world.entity.ai.goal.target.NearestAttackableTargetGoal;
import net.minecraft.world.level.GameType;
import net.minecraft.world.level.block.Blocks;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class SharkThreatProbe {
    private record Case(String name, double x, boolean baby, boolean owner, boolean creative) {}

    public SharkThreatProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        var command = Commands.literal("shark_threat_fixture");
        for (String mode : List.of("day", "night")) {
            command.then(Commands.literal(mode).executes(ctx -> {
            var src = ctx.getSource();
            try {
                var readings = new LinkedHashMap<String, Object>();
                var level = src.getLevel();
                level.setBlockAndUpdate(new BlockPos(7, 92, 6), Blocks.WATER.defaultBlockState());
                var contact = Entity.class.getDeclaredMethod("updateFluidInteraction");
                contact.setAccessible(true);
                var selectors = Mob.class.getDeclaredField("targetSelector");
                selectors.setAccessible(true);
                var player = player(level);
                for (var c : List.of(
                    new Case("dry_adult", 9.15, false, false, false),
                    new Case("water_adult", 7.75, false, false, false),
                    new Case("edge_adult", 8.12, false, false, false),
                    new Case("edge_baby", 8.12, true, false, false),
                    new Case("edge_creative", 8.12, false, false, true)
                )) {
                    player.setGameMode(c.creative ? GameType.CREATIVE : GameType.SURVIVAL);
                    player.snapTo(c.x, 92.35, 6.5, 0, 0);
                    contact.invoke(player);
                    var shark = NaturalistEntityTypes.GREAT_WHITE_SHARK.get().create(level, EntitySpawnReason.COMMAND);
                    shark.snapTo(11.0, 92.0, 6.5, 0, 0);
                    if (c.baby) shark.setAge(-24000);
                    level.addFreshEntity(shark);
                    readings.putIfAbsent("light", shark.getLightLevelDependentMagicValue());
                    var goal = ((GoalSelector) selectors.get(shark)).getAvailableGoals().stream()
                          .filter(entry -> entry.getPriority() == 3).map(WrappedGoal::getGoal)
                          .filter(NearestAttackableTargetGoal.class::isInstance).findFirst().orElseThrow();
                    boolean target = false;
                    for (int chance = 0; chance < 100 && !target; chance++) {
                        if (goal.canUse()) {
                            goal.start();
                            target = shark.getTarget() == player;
                        }
                    }
                    readings.put(c.name, List.of(player.isInWater(), target));
                    shark.discard();
                }
                player.discard();
                src.sendSuccess(() -> Component.literal("shark_threat_fixture " + new Gson().toJson(readings)), false);
            } catch (Exception error) {
                error.printStackTrace();
                src.sendSuccess(() -> Component.literal("shark_threat_error " + error), false);
            }
            return 1;
            }));
        }
        event.getDispatcher().register(command);
    }

    private static ServerPlayer player(ServerLevel level) {
        var profile = new GameProfile(UUID.randomUUID(), "SharkThreat");
        var player = new ServerPlayer(level.getServer(), level, profile, ClientInformation.createDefault());
        player.connection = new ServerGamePacketListenerImpl(level.getServer(), new Connection(PacketFlow.SERVERBOUND), player,
            CommonListenerCookie.createInitial(profile, false));
        player.snapTo(8.5, 92.35, 6.5, 0, 0);
        level.addNewPlayer(player);
        return player;
    }
}
