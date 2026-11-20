package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import net.minecraft.commands.Commands;
import net.minecraft.core.component.DataComponents;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.effect.MobEffects;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityTypes;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.entity.animal.fish.Cod;
import net.minecraft.world.item.component.CustomData;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class RayProbe {
    public RayProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("ray_fixture").executes(ctx -> {
            var source = ctx.getSource();
            ServerLevel level = source.getLevel();
            var ray = NaturalistEntityTypes.RAY.get().create(level, EntitySpawnReason.COMMAND);
            ray.snapTo(8.5, 92, 8.5, 0, 0);
            ray.setNoAi(true);
            level.addFreshEntity(ray);
            var attacker = add(level, 8.7);
            var target = add(level, 9.5);
            var protectedCod = add(level, 10.5);
            protectedCod.hurtServer(level, level.damageSources().mobAttack(attacker), 2.0F);
            boolean tookDamage = ray.hurtServer(level, level.damageSources().mobAttack(attacker), 1.0f);
            boolean retaliated = tookDamage && ray.getTarget() == attacker;
            ray.doHurtTarget(level, target);
            var poison = target.getEffect(MobEffects.POISON);
            ray.doHurtTarget(level, protectedCod);
            boolean protectedUnchanged = !protectedCod.hasEffect(MobEffects.POISON) && protectedCod.getHealth() == 1.0F;

            String defaultVariant = ray.getVariantString();
            ray.setVariantString("naturalist:mobula_ray");
            var stack = ray.getBucketItemStack();
            ray.saveToBucketTag(stack);
            var data = stack.getOrDefault(DataComponents.BUCKET_ENTITY_DATA, CustomData.EMPTY).copyTag();
            var custom = stack.getOrDefault(DataComponents.CUSTOM_DATA, CustomData.EMPTY).copyTag();
            var released = NaturalistEntityTypes.RAY.get().create(level, EntitySpawnReason.BUCKET);
            released.loadFromBucketTag(data);
            var legacy = data.copy();
            legacy.putInt("Variant", -1);
            var legacyFish = NaturalistEntityTypes.RAY.get().create(level, EntitySpawnReason.BUCKET);
            legacyFish.loadFromBucketTag(legacy);
            String result = """
                {"max_health":%s,"movement_speed":%s,"attack_damage":%s,"default_variant":"%s",\
                "retaliated":%s,"target_health":%s,"poison_duration":%s,"poison_amplifier":%s,"protected_unchanged":%s,\
                "bucket_variant":"%s","custom_variant":"%s","released_variant":"%s","legacy_variant":"%s"}
                """.strip().formatted(ray.getAttributeBaseValue(Attributes.MAX_HEALTH),
                    ray.getAttributeBaseValue(Attributes.MOVEMENT_SPEED), ray.getAttributeBaseValue(Attributes.ATTACK_DAMAGE),
                    defaultVariant, retaliated, target.getHealth(), poison == null ? -1 : poison.getDuration(),
                    poison == null ? -1 : poison.getAmplifier(), protectedUnchanged, data.getStringOr("Variant", ""),
                    custom.getStringOr("Variant", ""), released.getVariantString(), legacyFish.getVariantString());
            ray.discard();
            attacker.discard();
            target.discard();
            protectedCod.discard();
            source.sendSuccess(() -> Component.literal("ray_fixture " + result), false);
            return 1;
        }));
    }

    private static Cod add(ServerLevel level, double x) {
        var cod = EntityTypes.COD.create(level, EntitySpawnReason.COMMAND);
        cod.snapTo(x, 92, 8.5, 0, 0);
        cod.setNoAi(true);
        cod.setNoGravity(true);
        level.addFreshEntity(cod);
        return cod;
    }
}
