package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.entity.animal.boar.Boar;
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
public final class BoarStepProbe {
    public BoarStepProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("naturalist_fixture_boar_step").executes(context -> {
            var source = context.getSource();
            var boar = new RecordingBoar(source.getLevel());
            var rows = "[" + boar.step(Blocks.STONE, false) + "," + boar.step(Blocks.STONE, true) + "]";
            source.sendSuccess(() -> Component.literal("naturalist_fixture_boar_step " + rows), false);
            return 1;
        }));
    }

    private static final class RecordingBoar extends Boar {
        private String heard;

        RecordingBoar(Level level) {
            super(NaturalistEntityTypes.BOAR.get(), level);
        }

        String step(Block block, boolean baby) {
            setAge(baby ? -24000 : 0);
            playStepSound(BlockPos.ZERO, block.defaultBlockState());
            return heard;
        }

        @Override
        public void playSound(SoundEvent sound, float volume, float pitch) {
            heard = "[\"" + sound.location() + "\"," + volume + "," + pitch + "]";
        }
    }
}
