package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.entity.animal.vulture.Vulture;
import net.minecraft.commands.Commands;
import net.minecraft.network.chat.Component;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.effect.MobEffectInstance;
import net.minecraft.world.effect.MobEffects;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityTypes;
import net.minecraft.world.entity.item.ItemEntity;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.minecraft.world.phys.Vec3;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.common.util.FakePlayerFactory;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class VultureProbe {
    public VultureProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("vulture_fixture").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            var vulture = NaturalistEntityTypes.VULTURE.get().create(level, EntitySpawnReason.COMMAND);
            var zombie = EntityTypes.ZOMBIE.create(level, EntitySpawnReason.COMMAND);
            if (vulture == null || zombie == null) throw new IllegalStateException("Missing fixture mob");
            vulture.snapTo(8.5, 93.0, 8.5, 0, 0);
            level.addFreshEntity(vulture);
            zombie.snapTo(8.5, 93.0, 9.0, 0, 0);
            boolean cactusImmune = !vulture.hurtServer(level, vulture.damageSources().cactus(), 1f);
            boolean hungerImmune = !vulture.addEffect(new MobEffectInstance(MobEffects.HUNGER, 100));
            float attack = zombie.getHealth();
            vulture.doHurtTarget(level, zombie);
            attack -= zombie.getHealth();

            vulture.setPerched(true);
            vulture.startle(new Vec3(7.5, 94.0, 8.5));
            Vec3 impulse = vulture.getDeltaMovement();
            boolean startle = !vulture.isPerched() && !vulture.canPerch()
                    && impulse.distanceToSqr(new Vec3(0.4, 0.5, 0.0)) < 1.0E-6;
            vulture.setDeltaMovement(Vec3.ZERO);

            var player = FakePlayerFactory.getMinecraft(level);
            player.snapTo(8.5, 93.0, 9.0, 0, 0);
            player.setHealth(6f);
            player.setItemInHand(InteractionHand.MAIN_HAND, new ItemStack(Items.STICK, 3));
            boolean stole;
            int tookStack;
            boolean consumedAt601;
            try {
                var goalType = Class.forName(Vulture.class.getName() + "$VultureAttackGoal");
                var ctor = goalType.getDeclaredConstructors()[0];
                ctor.setAccessible(true);
                var goal = ctor.newInstance(vulture, 1.2, true);
                var hit = goalType.getDeclaredMethod("checkAndPerformAttack", net.minecraft.world.entity.LivingEntity.class);
                hit.setAccessible(true);
                vulture.setTarget(player);
                hit.invoke(goal, player);
                stole = player.getHealth() == 6f && player.getMainHandItem().getCount() == 2
                    && vulture.getMainHandItem().is(Items.STICK) && vulture.getMainHandItem().getCount() == 1;

                var pickUp = Vulture.class.getDeclaredMethod("pickUpItem", net.minecraft.server.level.ServerLevel.class, ItemEntity.class);
                pickUp.setAccessible(true);
                var flesh = new ItemEntity(level, 8.5, 93.9, 8.5, new ItemStack(Items.ROTTEN_FLESH, 3));
                pickUp.invoke(vulture, level, flesh);
                tookStack = flesh.isRemoved() && vulture.getMainHandItem().is(Items.ROTTEN_FLESH)
                    ? vulture.getMainHandItem().getCount() : 0;
                var eaten = Vulture.class.getDeclaredField("ticksSinceEaten");
                eaten.setAccessible(true);
                eaten.setInt(vulture, 599);
                vulture.aiStep();
                boolean heldAt600 = vulture.getMainHandItem().getCount() == 3;
                vulture.aiStep();
                consumedAt601 = heldAt600 && vulture.getMainHandItem().getCount() == 2;
            } catch (ReflectiveOperationException error) {
                throw new IllegalStateException(error);
            }

            String result = ("{\"cactus_immune\":%s,\"hunger_immune\":%s,"
                + "\"zombie_damage\":%.1f,\"startle\":%s,\"steals_one_without_damage\":%s,"
                + "\"took_rotten_flesh\":%d,\"eats_at_tick_601\":%s}")
                .formatted(cactusImmune, hungerImmune, attack, startle, stole, tookStack, consumedAt601);
            vulture.discard();
            source.sendSuccess(() -> Component.literal("vulture_fixture " + result), false);
            return 1;
        }));
    }
}
