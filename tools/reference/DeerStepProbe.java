package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.entity.animal.deer.Deer;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.network.chat.Component;
import net.minecraft.sounds.SoundEvent;
import net.minecraft.world.level.Level;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.Blocks;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class DeerStepProbe {
    public DeerStepProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("naturalist_fixture_deer_step").executes(context -> {
            var source = context.getSource();
            var deer = new RecordingDeer(source.getLevel());
            var rows = "[" + deer.step(Blocks.STONE) + "," + deer.step(Blocks.GRASS_BLOCK) + "]";
            source.sendSuccess(() -> Component.literal("naturalist_fixture_deer_step " + rows), false);
            return 1;
        }));
    }

    private static final class RecordingDeer extends Deer {
        private String heard;

        RecordingDeer(Level level) {
            super(NaturalistEntityTypes.DEER.get(), level);
        }

        String step(Block block) {
            playStepSound(BlockPos.ZERO, block.defaultBlockState());
            return heard;
        }

        @Override
        public void playSound(SoundEvent sound, float volume, float pitch) {
            heard = "[\"" + sound.location() + "\"," + volume + "," + pitch + "]";
        }
    }
}
