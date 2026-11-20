package naturalist.fixture;

import java.lang.reflect.Field;
import java.util.UUID;
import java.util.function.Predicate;

import com.crispytwig.naturalist.world.entity.animal.deer.Deer;
import com.mojang.authlib.GameProfile;
import net.minecraft.commands.CommandSourceStack;
import net.minecraft.commands.Commands;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.entity.LivingEntity;
import net.minecraft.world.entity.ai.goal.AvoidEntityGoal;
import net.minecraft.world.entity.ai.goal.WrappedGoal;
import net.minecraft.world.entity.player.Player;
import net.minecraft.world.level.entity.EntityTypeTest;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.common.util.FakePlayer;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

/** Test-only commands for reading original Naturalist 2.0.7 runtime behavior. */
@Mod("naturalist_fixture")
public final class DeerProbe {
    private static final Field PANIC_TICKS = field(Deer.class, "panicTicks");
    private static final Field AVOID_CLASS = field(AvoidEntityGoal.class, "avoidClass");
    private static final Field AVOID_PREDICATE = field(AvoidEntityGoal.class, "predicateOnAvoidEntity");

    public DeerProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("deer_fixture_probe")
                .then(Commands.literal("herd").executes(c -> herd(c.getSource())))
                .then(Commands.literal("sneak").executes(c -> sneaking(c.getSource()))));
    }

    private static Field field(Class<?> owner, String name) {
        try {
            Field field = owner.getDeclaredField(name);
            field.setAccessible(true);
            return field;
        } catch (ReflectiveOperationException e) {
            throw new IllegalStateException(e);
        }
    }

    private static Deer deer(ServerLevel level, String tag) {
        return level.getEntities(EntityTypeTest.forClass(Deer.class),
                d -> d.isAlive() && d.entityTags().contains(tag)).stream().findFirst().orElseThrow();
    }

    private static int timer(Deer deer) {
        try {
            return PANIC_TICKS.getInt(deer);
        } catch (IllegalAccessException e) {
            throw new IllegalStateException(e);
        }
    }

    private static int herd(CommandSourceStack source) {
        var level = source.getLevel();
        var hit = deer(level, "fixture_hit");
        var near = deer(level, "fixture_near");
        var far = deer(level, "fixture_far");
        String result = "%d %d %d %b".formatted(
                timer(hit), timer(near), timer(far), hit.getLastHurtByMob() != null);
        source.sendSuccess(() -> Component.literal("deer_fixture_herd " + result), false);
        return 1;
    }

    @SuppressWarnings("unchecked")
    private static boolean playerAvoidance(AvoidEntityGoal<?> goal, FakePlayer player, boolean sneak) {
        player.setShiftKeyDown(sneak);
        try {
            return ((Predicate<LivingEntity>) AVOID_PREDICATE.get(goal)).test(player);
        } catch (IllegalAccessException e) {
            throw new IllegalStateException(e);
        }
    }

    private static int sneaking(CommandSourceStack source) {
        var level = source.getLevel();
        var deer = deer(level, "fixture_sneak");
        var goal = deer.getGoalSelector().getAvailableGoals().stream()
                .map(WrappedGoal::getGoal)
                .filter(g -> g instanceof AvoidEntityGoal<?>)
                .map(g -> (AvoidEntityGoal<?>) g)
                .filter(g -> {
                    try {
                        return AVOID_CLASS.get(g) == Player.class;
                    } catch (IllegalAccessException e) {
                        throw new IllegalStateException(e);
                    }
                }).findFirst().orElseThrow();
        var player = new FakePlayer(level, new GameProfile(UUID.randomUUID(), "DeerFixture"));
        String result = "%b %b".formatted(
                playerAvoidance(goal, player, false),
                playerAvoidance(goal, player, true));
        source.sendSuccess(() -> Component.literal("deer_fixture_sneak " + result), false);
        return 1;
    }
}
