package naturalist.fixture;

import com.crispytwig.naturalist.world.entity.animal.deer.Deer;
import java.lang.reflect.Field;
import net.minecraft.commands.Commands;
import net.minecraft.network.chat.Component;
import net.minecraft.world.level.entity.EntityTypeTest;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class DeerProjectileProbe {
    private static final Field PANIC = panicField();

    public DeerProjectileProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private static Field panicField() {
        try {
            var field = Deer.class.getDeclaredField("panicTicks");
            field.setAccessible(true);
            return field;
        } catch (ReflectiveOperationException e) {
            throw new IllegalStateException(e);
        }
    }

    private static String record(Deer deer) {
        try {
            return "[" + PANIC.getInt(deer) + "," + deer.getHealth() + "]";
        } catch (IllegalAccessException e) {
            throw new IllegalStateException(e);
        }
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("naturalist_fixture_deer_projectiles")
            .executes(context -> {
                var source = context.getSource();
                var deer = source.getLevel().getEntities(EntityTypeTest.forClass(Deer.class),
                    mob -> mob.isAlive() && mob.entityTags().contains("projectile_herd"));
                deer.sort(java.util.Comparator.comparingDouble(Deer::getX));
                var rows = deer.stream().map(DeerProjectileProbe::record)
                    .collect(java.util.stream.Collectors.joining(",", "[", "]"));
                source.sendSuccess(() -> Component.literal("Naturalist projectile herd " + rows), false);
                return 1;
            }));
    }
}
