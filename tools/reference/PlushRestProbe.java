package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistRegistry;
import com.mojang.authlib.GameProfile;
import com.mojang.brigadier.arguments.StringArgumentType;
import java.util.UUID;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.network.Connection;
import net.minecraft.resources.Identifier;
import net.minecraft.network.chat.Component;
import net.minecraft.network.protocol.PacketFlow;
import net.minecraft.server.level.ClientInformation;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.server.network.CommonListenerCookie;
import net.minecraft.server.network.ServerGamePacketListenerImpl;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.entity.EntityReference;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.Mob;
import net.minecraft.world.entity.monster.zombie.ZombifiedPiglin;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.minecraft.world.level.GameType;
import net.minecraft.world.level.block.AbstractBedBlock;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.state.properties.BedPart;
import net.minecraft.world.phys.BlockHitResult;
import net.minecraft.world.phys.Vec3;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class PlushRestProbe {
    public PlushRestProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("plush_rest_probe")
            .then(Commands.argument("case", StringArgumentType.word()).executes(context -> {
                var name = StringArgumentType.getString(context, "case");
                boolean slept = useBed(context.getSource().getLevel(), name);
                context.getSource().sendSuccess(() -> Component.literal("plush_rest_probe " + name + " " + slept), false);
                return 1;
            })));
    }

    private static boolean useBed(ServerLevel level, String name) {
        var foot = new BlockPos(8, 102, 8);
        var head = foot.south();
        var bed = BuiltInRegistries.BLOCK.getValue(Identifier.parse("minecraft:red_bed")).defaultBlockState().setValue(AbstractBedBlock.FACING, Direction.SOUTH);
        level.setBlock(head, bed.setValue(AbstractBedBlock.PART, BedPart.HEAD), Block.UPDATE_CLIENTS);
        level.setBlock(foot, bed.setValue(AbstractBedBlock.PART, BedPart.FOOT), Block.UPDATE_CLIENTS);

        var profile = new GameProfile(UUID.randomUUID(), "PlushFixture");
        var player = new ServerPlayer(level.getServer(), level, profile, ClientInformation.createDefault());
        player.connection = new ServerGamePacketListenerImpl(level.getServer(), new Connection(PacketFlow.SERVERBOUND),
            player, CommonListenerCookie.createInitial(profile, false));
        player.snapTo(8.5, 102.0, 6.5, 0.0F, 0.0F);
        level.addNewPlayer(player);
        player.setGameMode(GameType.SURVIVAL);
        String kind = name.contains("piglin") ? "minecraft:zombified_piglin" : "minecraft:zombie";
        var zombie = name.equals("empty_no_monsters") ? null :
            BuiltInRegistries.ENTITY_TYPE.getValue(Identifier.parse(kind)).create(level, EntitySpawnReason.COMMAND);
        if (zombie != null) {
            zombie.snapTo(9.5, 102.0, 11.5, 0.0F, 0.0F);
            if (zombie instanceof Mob mob) {
                mob.setNoAi(true);
            }
            level.addFreshEntity(zombie);
            if (zombie instanceof ZombifiedPiglin piglin && name.equals("angry_piglin_plush")) {
                piglin.setPersistentAngerTarget(EntityReference.of(player));
                piglin.setTimeToRemainAngry(600);
                if (!piglin.isAngryAt(player, level)) {
                    throw new IllegalStateException("Fixture Piglin did not acquire anger");
                }
            }
        }
        var bear = new ItemStack(NaturalistRegistry.PLUSH_BEAR.get().asItem());
        switch (name) {
            case "main_nearby", "day_plush", "angry_piglin_plush", "calm_piglin_plush", "blocked_plush" -> player.setItemInHand(InteractionHand.MAIN_HAND, bear);
            case "offhand_nearby" -> player.setItemInHand(InteractionHand.OFF_HAND, bear);
            case "inventory_nearby" -> player.getInventory().setItem(8, bear);
            case "ordinary_item_nearby" -> player.setItemInHand(InteractionHand.MAIN_HAND, new ItemStack(Items.STICK));
        }
        try {
            if (name.equals("blocked_plush")) {
                var stone = BuiltInRegistries.BLOCK.getValue(Identifier.parse("minecraft:stone")).defaultBlockState();
                level.setBlock(head.above(), stone, Block.UPDATE_CLIENTS);
            }
            var hit = new BlockHitResult(new Vec3(8.5, 102.5625, 8.5), Direction.UP, foot, false);
            level.getBlockState(foot).useWithoutItem(level, player, hit);
            return player.isSleeping();
        } finally {
            if (player.isSleeping()) {
                player.stopSleepInBed(true, false);
            }
            player.discard();
            if (zombie != null) {
                zombie.discard();
            }
        }
    }
}
