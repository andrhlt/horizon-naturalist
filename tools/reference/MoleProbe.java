package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.entity.WolfMoleDigging;
import com.crispytwig.naturalist.world.entity.ai.goal.WolfDigOutMoleGoal;
import com.crispytwig.naturalist.world.entity.animal.mole.DirtTrail;
import com.crispytwig.naturalist.world.entity.animal.mole.Mole;
import com.google.gson.Gson;
import java.lang.reflect.Method;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.util.ProblemReporter;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityTypes;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.storage.TagValueOutput;
import net.minecraft.world.phys.AABB;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class MoleProbe {
    public MoleProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("naturalist_mole_fixture").executes(context -> {
            var source = context.getSource();
            try {
                var records = capture(source.getLevel());
                source.sendSuccess(() -> Component.literal("naturalist_mole_fixture " + new Gson().toJson(records)), false);
                return 1;
            } catch (ReflectiveOperationException failure) {
                throw new IllegalStateException(failure);
            }
        }));
    }

    private static Map<String, Object> capture(ServerLevel level) throws ReflectiveOperationException {
        var record = new LinkedHashMap<String, Object>();
        for (int x = 3; x <= 12; x++) {
            for (int z = 3; z <= 12; z++) {
                level.setBlockAndUpdate(new BlockPos(x, 90, z), Blocks.STONE.defaultBlockState());
                level.setBlockAndUpdate(new BlockPos(x, 91, z), Blocks.AIR.defaultBlockState());
            }
        }
        Mole mole = NaturalistEntityTypes.MOLE.get().create(level, EntitySpawnReason.COMMAND);
        mole.snapTo(8.2, 91, 8.8, 0, 0);
        mole.setOnGround(true);
        level.addFreshEntity(mole);
        var setState = method("setState", int.class);
        var getState = method("getState");
        var step = method("customServerAiStep", ServerLevel.class);
        var zombie = EntityTypes.ZOMBIE.create(level, EntitySpawnReason.COMMAND);
        try {
            record.put("default_variant", mole.getVariantString());
            record.put("adult_dimensions", List.of(mole.getBbWidth(), mole.getBbHeight(), mole.getEyeHeight()));
            record.put("attributes", List.of(mole.getMaxHealth(), mole.getAttributeBaseValue(Attributes.MOVEMENT_SPEED),
                mole.getAttributeBaseValue(Attributes.STEP_HEIGHT)));
            record.put("food", List.of(mole.isFood(new ItemStack(Items.SPIDER_EYE)), mole.isFood(new ItemStack(Items.FERMENTED_SPIDER_EYE)),
                mole.isFood(new ItemStack(Items.APPLE))));
            mole.setAge(-24000);
            record.put("baby_dimensions", List.of(mole.getBbWidth(), mole.getBbHeight(), mole.getEyeHeight()));
            mole.setAge(0);
            var states = new LinkedHashMap<String, Object>();
            for (int phase = 0; phase <= 4; phase++) {
                Mole trial = NaturalistEntityTypes.MOLE.get().create(level, EntitySpawnReason.COMMAND);
                trial.snapTo(4.2 + phase, 91, 4.8, 0, 0);
                trial.setOnGround(true);
                setState.invoke(trial, phase);
                var saved = TagValueOutput.createWithContext(ProblemReporter.DISCARDING, level.registryAccess());
                trial.addAdditionalSaveData(saved);
                boolean hurt = trial.hurtServer(level, level.damageSources().generic(), 1);
                states.put(Integer.toString(phase), Map.of("saved", saved.buildResult().getIntOr("MoleState", -1),
                    "hide_goal", trial.canHide(), "pushable", trial.isPushable(), "hurt", hurt,
                    "step_height", trial.getAttributeBaseValue(Attributes.STEP_HEIGHT)));
                trial.discard();
            }
            record.put("states", states);
            zombie.snapTo(10.5, 91, 8.5, 0, 0);
            zombie.setNoAi(true);
            level.addFreshEntity(zombie);
            step.invoke(mole, level);
            record.put("undead_triggers_dig", getState.invoke(mole));
            record.put("dig_centres", List.of(mole.getX(), mole.getZ()));
            for (int i = 0; i < 19; i++) step.invoke(mole, level);
            record.put("dig_tick_19", getState.invoke(mole));
            step.invoke(mole, level);
            record.put("dig_tick_20", getState.invoke(mole));
            var trails = level.getEntitiesOfClass(DirtTrail.class, new AABB(new BlockPos(8, 91, 8)));
            record.put("mound_count", trails.size());
            record.put("mound_persisted", trails.getFirst().shouldBeSaved());
            record.put("mound_small", trails.getFirst().isSmall());
            record.put("mound_hurt", trails.getFirst().hurtServer(level, level.damageSources().generic(), 1));
            for (int i = 1; i <= 49; i++) {
                trails.getFirst().tickCount = i;
                trails.getFirst().tick();
            }
            record.put("mound_at_49", !trails.getFirst().isRemoved());
            trails.getFirst().tickCount = 50;
            trails.getFirst().tick();
            record.put("mound_at_50", !trails.getFirst().isRemoved());
            for (int i = 0; i < 59; i++) step.invoke(mole, level);
            record.put("diggable_tick_59", mole.canBeDugOut());
            step.invoke(mole, level);
            record.put("diggable_tick_60", mole.canBeDugOut());
            zombie.discard();
            for (int i = 0; i < 59; i++) step.invoke(mole, level);
            record.put("calm_tick_59", getState.invoke(mole));
            step.invoke(mole, level);
            record.put("calm_tick_60", getState.invoke(mole));
            for (int i = 0; i < 56; i++) step.invoke(mole, level);
            record.put("peek_tick_56", getState.invoke(mole));
            step.invoke(mole, level);
            record.put("peek_tick_57", getState.invoke(mole));
            for (int i = 0; i < 24; i++) step.invoke(mole, level);
            record.put("emerge_tick_24", getState.invoke(mole));
            step.invoke(mole, level);
            record.put("emerge_tick_25", getState.invoke(mole));
            record.put("wolf_dig", wolfDig(level, mole, setState, getState, step));

            return record;
        } finally {
            mole.discard();
            zombie.discard();
        }
    }

    private static Map<String, Object> wolfDig(
        ServerLevel level, Mole mole, Method setState, Method getState, Method step
    ) throws ReflectiveOperationException {
        var result = new LinkedHashMap<String, Object>();
        var wolf = EntityTypes.WOLF.create(level, EntitySpawnReason.COMMAND);
        wolf.snapTo(10.49, 91, 8.5, 0, 0);
        wolf.setOnGround(true);
        wolf.setTarget(mole);
        level.addFreshEntity(wolf);
        try {
            var goal = new WolfDigOutMoleGoal(wolf);
            setState.invoke(mole, 2);
            var scan = Mole.class.getDeclaredField("threatDetected");
            scan.setAccessible(true);
            var remembered = Mole.class.getDeclaredField("rememberedAttacker");
            remembered.setAccessible(true);
            var memory = Mole.class.getDeclaredField("attackerMemoryTicks");
            memory.setAccessible(true);
            remembered.set(mole, wolf.getUUID());
            memory.setInt(mole, 1200);
            for (int i = 0; i < 59; i++) step.invoke(mole, level);
            result.put("grace_tick_59", goal.canUse());
            step.invoke(mole, level);
            result.put("grace_tick_60", goal.canUse());
            wolf.snapTo(10.51, 91, 8.5, 0, 0);
            result.put("starts_over_two", goal.canUse());
            wolf.snapTo(10.49, 91, 8.5, 0, 0);
            goal.start();
            result.put("digging_pose_during", ((WolfMoleDigging) wolf).naturalist$isDiggingOutMole());
            wolf.snapTo(11.01, 91, 8.5, 0, 0);
            result.put("continues_over_two_point_five", goal.canContinueToUse());
            wolf.snapTo(10.99, 91, 8.5, 0, 0);
            result.put("continues_inside_two_point_five", goal.canContinueToUse());
            for (int i = 0; i < 49; i++) goal.tick();
            result.put("phase_at_49", getState.invoke(mole));
            result.put("holds_mole", !mole.isBeingDugOut());
            goal.tick();
            result.put("phase_at_50", getState.invoke(mole));
            result.put("anger_target", wolf.getUUID().equals(remembered.get(mole)));
            result.put("ends_goal", !goal.canContinueToUse());
            goal.stop();
            result.put("digging_pose_after", ((WolfMoleDigging) wolf).naturalist$isDiggingOutMole());
            return result;
        } finally {
            wolf.discard();
        }
    }

    private static Method method(String name, Class<?>... params) throws ReflectiveOperationException {
        var call = Mole.class.getDeclaredMethod(name, params);
        call.setAccessible(true);
        return call;
    }
}
