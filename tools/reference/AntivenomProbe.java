package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistMobEffects;
import com.crispytwig.naturalist.registry.NaturalistPotions;
import com.google.gson.Gson;
import java.util.LinkedHashMap;
import net.minecraft.commands.CommandSourceStack;
import net.minecraft.commands.Commands;
import net.minecraft.core.component.DataComponents;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.effect.MobEffectInstance;
import net.minecraft.world.effect.MobEffects;
import net.minecraft.world.entity.AreaEffectCloud;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityTypes;
import net.minecraft.world.entity.LivingEntity;
import net.minecraft.world.entity.projectile.arrow.Arrow;
import net.minecraft.world.entity.projectile.throwableitemprojectile.ThrownLingeringPotion;
import net.minecraft.world.entity.projectile.throwableitemprojectile.ThrownSplashPotion;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.minecraft.world.item.alchemy.PotionContents;
import net.minecraft.world.phys.EntityHitResult;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class AntivenomProbe {
    public AntivenomProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("naturalist_fixture_antivenom")
                .executes(ctx -> capture(ctx.getSource())));
    }

    private static int capture(CommandSourceStack source) {
        ServerLevel level = source.getLevel();
        var target = EntityTypes.PIG.create(level, EntitySpawnReason.COMMAND);
        target.snapTo(7.5D, 72D, 7.5D, 0, 0);
        target.setNoAi(true);
        level.addFreshEntity(target);
        var contents = new PotionContents(BuiltInRegistries.POTION.wrapAsHolder(NaturalistPotions.ANTIVENOM.get()));
        var result = new LinkedHashMap<String, Object>();
        result.put("color", NaturalistMobEffects.ANTIVENOM.get().getColor());
        result.put("instant", NaturalistMobEffects.ANTIVENOM.get().isInstantaneous());
        try {
            poison(target);
            contents.applyToLivingEntity(target, 1.0F);
            result.put("drink_removes_poison", !target.hasEffect(MobEffects.POISON));
            result.put("drink_leaves_regeneration", target.hasEffect(MobEffects.REGENERATION));

            poison(target);
            var splash = new ItemStack(Items.SPLASH_POTION);
            splash.set(DataComponents.POTION_CONTENTS, contents);
            var hit = new EntityHitResult(target);
            var bottle = new ThrownSplashPotion(level, 7.5D, 72D, 7.5D, splash);
            bottle.onHitAsPotion(level, splash, hit);
            result.put("splash_removes_poison", !target.hasEffect(MobEffects.POISON));
            result.put("splash_leaves_regeneration", target.hasEffect(MobEffects.REGENERATION));

            poison(target);
            var lingering = new ItemStack(Items.LINGERING_POTION);
            lingering.set(DataComponents.POTION_CONTENTS, contents);
            var thrown = new ThrownLingeringPotion(level, 7.5D, 72D, 7.5D, lingering);
            thrown.onHitAsPotion(level, lingering, hit);
            var cloud = level.getEntitiesOfClass(AreaEffectCloud.class, target.getBoundingBox().inflate(4))
                    .stream().findFirst().orElseThrow();
            result.put("lingering_waits_before_removing_poison", target.hasEffect(MobEffects.POISON));
            for (int i = 0; i < 15; i++) {
                cloud.tickCount++;
                cloud.tick();
            }
            result.put("lingering_removes_poison", !target.hasEffect(MobEffects.POISON));
            result.put("lingering_leaves_regeneration", target.hasEffect(MobEffects.REGENERATION));
            cloud.discard();

            poison(target);
            var tipped = new ItemStack(Items.TIPPED_ARROW);
            tipped.set(DataComponents.POTION_CONTENTS, contents);
            new ProbeArrow(level, tipped).apply(target);
            target.tick();
            result.put("arrow_removes_poison", !target.hasEffect(MobEffects.POISON));

            poison(target);
            target.addEffect(new MobEffectInstance(BuiltInRegistries.MOB_EFFECT.wrapAsHolder(NaturalistMobEffects.ANTIVENOM.get()), 1));
            target.tick();
            result.put("direct_effect_removes_poison", !target.hasEffect(MobEffects.POISON));
        } finally {
            target.discard();
        }
        source.sendSuccess(() -> Component.literal("naturalist_antivenom " + new Gson().toJson(result)), false);
        return 1;
    }

    private static void poison(LivingEntity target) {
        target.addEffect(new MobEffectInstance(MobEffects.POISON, 320));
        target.addEffect(new MobEffectInstance(MobEffects.REGENERATION, 320));
    }

    private static final class ProbeArrow extends Arrow {
        ProbeArrow(ServerLevel level, ItemStack tipped) {
            super(level, 7.5D, 72D, 7.5D, tipped, null);
        }

        void apply(LivingEntity target) {
            doPostHurtEffects(target);
        }
    }
}
