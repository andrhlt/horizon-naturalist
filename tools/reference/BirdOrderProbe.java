package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.entity.animal.bird.Bird;
import net.minecraft.commands.Commands;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.ai.goal.Goal;
import net.minecraft.world.entity.ai.goal.WrappedGoal;
import net.minecraft.world.entity.item.ItemEntity;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class BirdOrderProbe {
    public BirdOrderProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private static Bird bird(ServerLevel level, double x, int age) {
        var bird = NaturalistEntityTypes.BIRD.get().create(level, EntitySpawnReason.COMMAND);
        if (bird == null) throw new IllegalStateException("Bird did not register");
        bird.snapTo(x, 93.0, 8.0, 0.0f, 0.0f);
        bird.setAge(age);
        if (!level.addFreshEntity(bird)) throw new IllegalStateException("Bird did not spawn");
        return bird;
    }

    private static Goal goal(Bird bird, String name) {
        return bird.getGoalSelector().getAvailableGoals().stream()
            .map(WrappedGoal::getGoal)
            .filter(goal -> goal.getClass().getSimpleName().equals(name))
            .findFirst().orElseThrow();
    }

    private static Object selected(Goal goal, Class<?> declaringClass, String fieldName) {
        try {
            var field = declaringClass.getDeclaredField(fieldName);
            field.setAccessible(true);
            return field.get(goal);
        } catch (ReflectiveOperationException cause) {
            throw new IllegalStateException(cause);
        }
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("bird_fixture_order").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            var leader = bird(level, 2.5, 0);
            var adultFlock = goal(leader, "BirdFlockGoal");
            var canFollowSelf = adultFlock.canUse()
                && selected(adultFlock, adultFlock.getClass().getSuperclass(), "followingMob") == leader;
            var near = bird(level, 11.5, 0);
            var young = bird(level, 10.5, -24000);
            var flock = goal(young, "BirdFlockGoal");
            var firstLeader = flock.canUse()
                && selected(flock, flock.getClass().getSuperclass(), "followingMob") == leader;

            var seeds = new ItemEntity(level, 11.4, 93.0, 8.0, new ItemStack(Items.PUMPKIN_SEEDS, 3));
            seeds.setPickUpDelay(32767);
            if (!level.addFreshEntity(seeds)) throw new IllegalStateException("Seeds did not spawn");
            var foraging = goal(young, "BirdEatSeedsGoal");
            if (!foraging.canUse()) throw new IllegalStateException("Bird ignored seeds");
            boolean choosesSeeds = selected(foraging, foraging.getClass(), "targetSeeds") == seeds;
            foraging.tick();
            int afterFirst = seeds.getItem().getCount();
            for (int i = 0; i < 20; i++) foraging.tick();
            int afterCooldown = seeds.getItem().getCount();
            foraging.tick();
            int afterSecond = seeds.getItem().getCount();
            boolean staysWild = !young.isTame();
            seeds.discard();
            young.discard();
            near.discard();
            leader.discard();
            var earlyJoin = bird(level, 17.5, 0);
            var earlySection = bird(level, 8.5, 0);
            var across = bird(level, 16.5, -24000);
            var acrossFlock = goal(across, "BirdFlockGoal");
            var lowSectionFirst = acrossFlock.canUse()
                && selected(acrossFlock, acrossFlock.getClass().getSuperclass(), "followingMob") == earlySection;
            earlyJoin.discard();
            earlySection.discard();
            across.discard();
            var result = ("{\"adult_selects_self\":%s,\"juvenile_selects_first_adult\":%s," +
                          "\"chooses_seeds\":%s,\"after_first\":%d,\"after_cooldown\":%d," +
                          "\"after_second\":%d,\"ownerless_stays_wild\":%s,\"lower_section_first\":%s}")
                         .formatted(canFollowSelf, firstLeader, choosesSeeds, afterFirst,
                                    afterCooldown, afterSecond, staysWild, lowSectionFirst);
            source.sendSuccess(() -> Component.literal("bird_fixture_order " + result), false);
            return 1;
        }));
    }
}
