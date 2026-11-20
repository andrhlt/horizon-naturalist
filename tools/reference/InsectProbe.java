package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.registry.NaturalistRegistry;
import com.crispytwig.naturalist.world.entity.animal.butterfly.Butterfly;
import com.crispytwig.naturalist.world.entity.animal.butterfly.Caterpillar;
import com.crispytwig.naturalist.world.level.block.ChrysalisBlock;
import java.util.ArrayList;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.ai.goal.Goal;
import net.minecraft.world.entity.ai.goal.MoveToBlockGoal;
import net.minecraft.world.entity.ai.goal.WrappedGoal;
import net.minecraft.world.entity.Mob;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.CropBlock;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class InsectProbe {
    public InsectProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private static Goal goal(Mob mob, String name) {
        return mob.getGoalSelector().getAvailableGoals().stream()
            .map(WrappedGoal::getGoal)
            .filter(goal -> goal.getClass().getSimpleName().equals(name))
            .findFirst().orElseThrow();
    }

    private static void target(Goal goal, BlockPos pos) {
        try {
            var field = MoveToBlockGoal.class.getDeclaredField("blockPos");
            field.setAccessible(true);
            field.set(goal, pos);
        } catch (ReflectiveOperationException cause) {
            throw new IllegalStateException(cause);
        }
    }

    private static String run(ServerLevel level) {
        var butterfly = NaturalistEntityTypes.BUTTERFLY.get().create(level, EntitySpawnReason.COMMAND);
        var caterpillar = NaturalistEntityTypes.CATERPILLAR.get().create(level, EntitySpawnReason.COMMAND);
        if (butterfly == null || caterpillar == null) throw new IllegalStateException("Missing insects");
        boolean breedsCaterpillar = butterfly.getBreedOffspring(level, butterfly) instanceof Caterpillar;
        var flower = new BlockPos(4, 93, 4);
        level.setBlockAndUpdate(flower, Blocks.DANDELION.defaultBlockState());
        butterfly.snapTo(4.5, 94.5, 4.5, 0, 0);
        var pollinate = goal(butterfly, "ButterflyPollinateGoal");
        target(pollinate, flower);
        for (int i = 0; i < 40; i++) pollinate.tick();
        boolean beforeFortyOne = butterfly.hasNectar();
        pollinate.tick();
        boolean onFortyOne = butterfly.hasNectar();

        var wheat = new BlockPos(5, 93, 4);
        level.setBlockAndUpdate(wheat.below(), Blocks.FARMLAND.defaultBlockState());
        var crop = (CropBlock) Blocks.WHEAT;
        butterfly.snapTo(5.5, 94.5, 4.5, 0, 0);
        var grow = goal(butterfly, "ButterflyGrowCropGoal");
        target(grow, wheat);
        boolean grewEachCrop = true;
        for (int i = 0; i < 10; i++) {
            level.setBlockAndUpdate(wheat, crop.defaultBlockState());
            grow.tick();
            grewEachCrop &= crop.getAge(level.getBlockState(wheat)) >= 2;
        }
        boolean beforeStep = butterfly.hasNectar();
        butterfly.setNoAi(true);
        butterfly.aiStep();
        boolean afterStep = butterfly.hasNectar();

        var shell = new BlockPos(9, 94, 8);
        level.setBlockAndUpdate(shell.east(), Blocks.OAK_LOG.defaultBlockState());
        level.setBlockAndUpdate(shell.east().below(), Blocks.OAK_LOG.defaultBlockState());
        caterpillar.snapTo(9.5, 94, 8.5, 0, 0);
        if (!level.addFreshEntity(caterpillar)) throw new IllegalStateException("Caterpillar did not spawn");
        var cocoon = goal(caterpillar, "CocoonGoal");
        caterpillar.setAge(-24000);
        boolean babyMakesCocoon = cocoon.canUse();
        caterpillar.setAge(0);
        boolean adultMakesCocoon = cocoon.canUse();
        if (adultMakesCocoon) cocoon.tick();
        var chrysalis = NaturalistRegistry.CHRYSALIS_BLOCK.get();
        var afterCocoon = level.getBlockState(shell);
        boolean removed = caterpillar.isRemoved();
        boolean facesLog = afterCocoon.is(chrysalis) && afterCocoon.getValue(ChrysalisBlock.FACING) == Direction.EAST;
        var ages = new ArrayList<Integer>();
        int previous = -1;
        for (int i = 0; i < 100 && level.getBlockState(shell).is(chrysalis); i++) {
            var state = level.getBlockState(shell);
            int age = state.getValue(ChrysalisBlock.AGE);
            if (age != previous) ages.add(age);
            previous = age;
            chrysalis.randomTick(state, level, shell, level.getRandom());
        }
        var hatched = level.getEntitiesOfClass(Butterfly.class, new AABB(shell).inflate(0.01));
        boolean hatchAtCenter = hatched.size() == 1 && hatched.getFirst().position().equals(Vec3.atCenterOf(shell));
        hatched.forEach(Butterfly::discard);
        butterfly.discard();
        level.removeBlock(shell.east(), false);
        level.removeBlock(shell.east().below(), false);
        return ("{\"breeds_caterpillar\":%s,\"nectar_after_40_goal_ticks\":%s,\"nectar_after_41_goal_ticks\":%s,"
              + "\"grew_each_crop\":%s,\"nectar_after_tenth_crop_before_step\":%s,\"nectar_after_tenth_crop_after_step\":%s,"
              + "\"baby_makes_cocoon\":%s,\"adult_makes_cocoon\":%s,\"caterpillar_discarded\":%s,"
              + "\"chrysalis_faces_log\":%s,\"chrysalis_ages\":%s,\"butterfly_hatches_at_center\":%s}")
            .formatted(breedsCaterpillar, beforeFortyOne, onFortyOne, grewEachCrop, beforeStep, afterStep,
                       babyMakesCocoon, adultMakesCocoon, removed, facesLog, ages, hatchAtCenter);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("insect_fixture_lifecycle").executes(ctx -> {
            var source = ctx.getSource();
            var result = run(source.getLevel());
            source.sendSuccess(() -> Component.literal("insect_fixture_lifecycle " + result), false);
            return 1;
        }));
    }
}
