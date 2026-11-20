package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.entity.animal.bird.Bird;
import java.lang.reflect.Method;
import java.util.ArrayList;
import java.util.List;
import java.util.Locale;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.network.chat.Component;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.state.BlockState;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.PlayLevelSoundEvent;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class BirdSoundProbe {
    private final List<String> events = new ArrayList<>();
    private boolean recording;

    public BirdSoundProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
        NeoForge.EVENT_BUS.addListener(this::atPosition);
        NeoForge.EVENT_BUS.addListener(this::atEntity);
    }

    private void atPosition(PlayLevelSoundEvent.AtPosition event) { observe(event); }
    private void atEntity(PlayLevelSoundEvent.AtEntity event) { observe(event); }

    private void observe(PlayLevelSoundEvent event) {
        if (recording) {
            events.add(String.format(Locale.ROOT, "{\"sound\":\"%s\",\"volume\":%.4f,\"pitch\":%.4f}",
                event.getSound().value().location(), event.getOriginalVolume(), event.getOriginalPitch()));
        }
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("bird_sound_fixture").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            var bird = NaturalistEntityTypes.BIRD.get().create(level, EntitySpawnReason.COMMAND);
            if (bird == null) throw new IllegalStateException("Missing fixture Bird");
            var position = new BlockPos(8, 92, 8);
            bird.snapTo(8.5, 93.0, 8.5, 0.0F, 0.0F);
            var state = Blocks.STONE.defaultBlockState();
            level.setBlockAndUpdate(position, state);
            events.clear();
            recording = true;
            try {
                method(Bird.class, "playStepSound", BlockPos.class, BlockState.class).invoke(bird, position, state);
                method(Bird.class, "onFlap").invoke(bird);
                bird.setSilent(true);
                method(Bird.class, "onFlap").invoke(bird);
                var json = "[" + String.join(",", events) + "]";
                source.sendSuccess(() -> Component.literal("bird_sound_fixture " + json), false);
                return 1;
            } catch (ReflectiveOperationException error) {
                throw new IllegalStateException(error);
            } finally {
                recording = false;
            }
        }));
    }

    private static Method method(Class<?> type, String name, Class<?>... parameters) throws ReflectiveOperationException {
        for (Class<?> owner = type; owner != null; owner = owner.getSuperclass()) {
            try {
                Method found = owner.getDeclaredMethod(name, parameters);
                found.setAccessible(true);
                return found;
            } catch (NoSuchMethodException omitted) {
                // Try the next inherited implementation.
            }
        }
        throw new NoSuchMethodException(name);
    }
}
