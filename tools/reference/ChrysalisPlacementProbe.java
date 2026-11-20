package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistRegistry;
import com.crispytwig.naturalist.world.level.block.ChrysalisBlock;
import com.mojang.authlib.GameProfile;
import java.util.UUID;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;
import net.minecraft.network.chat.Component;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.context.BlockPlaceContext;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.phys.BlockHitResult;
import net.minecraft.world.phys.Vec3;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.common.util.FakePlayer;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class ChrysalisPlacementProbe {
    public ChrysalisPlacementProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("chrysalis_placement_fixture").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            var player = new FakePlayer(level, new GameProfile(UUID.randomUUID(), "PlaceFixture"));
            player.snapTo(8.5, 93, 6.5, 0, 0);
            var stone = new BlockPos(7, 94, 7);
            var oak = new BlockPos(11, 94, 7);
            level.setBlockAndUpdate(stone, Blocks.STONE.defaultBlockState());
            level.setBlockAndUpdate(oak, Blocks.OAK_LOG.defaultBlockState());
            var block = NaturalistRegistry.CHRYSALIS_BLOCK.get();
            var held = new ItemStack(block);
            var badClick = new BlockHitResult(Vec3.atCenterOf(stone), Direction.EAST, stone, false);
            var bad = block.getStateForPlacement(new BlockPlaceContext(level, player, InteractionHand.MAIN_HAND, held, badClick));
            var goodClick = new BlockHitResult(Vec3.atCenterOf(oak), Direction.WEST, oak, false);
            var good = block.getStateForPlacement(new BlockPlaceContext(level, player, InteractionHand.MAIN_HAND, held, goodClick));
            boolean eastOnLog = good != null && good.getValue(ChrysalisBlock.FACING) == Direction.EAST
                && good.getValue(ChrysalisBlock.AGE) == 0;
            String json = "{\"refuses_stone\":" + (bad == null) + ",\"east_on_log\":" + eastOnLog + "}";
            source.sendSuccess(() -> Component.literal("chrysalis_placement_fixture " + json), false);
            return 1;
        }));
    }
}
