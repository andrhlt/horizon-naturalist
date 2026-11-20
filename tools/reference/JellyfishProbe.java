package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.entity.animal.jellyfish.Jellyfish;
import net.minecraft.commands.Commands;
import net.minecraft.core.component.DataComponents;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityTypes;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.item.component.CustomData;
import net.minecraft.world.phys.Vec3;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class JellyfishProbe {
    public JellyfishProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("jellyfish_fixture").executes(ctx -> {
            var source = ctx.getSource();
            ServerLevel level = source.getLevel();
            var fish = add(level, 8.5);
            var peer = add(level, 8.7);
            var cod = EntityTypes.COD.create(level, EntitySpawnReason.COMMAND);
            cod.snapTo(8.7, 92.0, 8.5, 0, 0);
            cod.setNoAi(true);
            level.addFreshEntity(cod);
            var remoteCod = EntityTypes.COD.create(level, EntitySpawnReason.COMMAND);
            remoteCod.snapTo(10.5, 92.0, 8.5, 0, 0);
            remoteCod.setNoAi(true);
            level.addFreshEntity(remoteCod);

            fish.getMoveControl().setWantedPosition(12.5, 92.0, 8.5, 1.0);
            boolean first = pulse(fish);
            float touched = cod.getHealth();
            float untouched = remoteCod.getHealth();
            float ally = peer.getHealth();
            boolean noEarly = true;
            for (int tick = 0; tick < 19; tick++) {
                noEarly &= !pulse(fish);
            }
            boolean second = pulse(fish);
            String defaultVariant = fish.getVariantString();
            fish.setVariantString("naturalist:green");
            var stack = fish.getBucketItemStack();
            fish.saveToBucketTag(stack);
            var data = stack.getOrDefault(DataComponents.BUCKET_ENTITY_DATA, CustomData.EMPTY).copyTag();
            var custom = stack.getOrDefault(DataComponents.CUSTOM_DATA, CustomData.EMPTY).copyTag();
            var released = NaturalistEntityTypes.JELLYFISH.get().create(level, EntitySpawnReason.BUCKET);
            released.loadFromBucketTag(data);
            var legacy = data.copy();
            legacy.putInt("Variant", -2);
            var legacyFish = NaturalistEntityTypes.JELLYFISH.get().create(level, EntitySpawnReason.BUCKET);
            legacyFish.loadFromBucketTag(legacy);
            String result = """
                {"max_health":%s,"movement_speed":%s,"default_variant":"%s",\
                "first_pulse":%s,"no_early_pulse":%s,"second_pulse":%s,\
                "touching_cod":%s,"outside_cod":%s,"nearby_jellyfish":%s,\
                "bucket_variant":"%s","custom_variant":"%s","released_variant":"%s","legacy_variant":"%s"}
                """.strip().formatted(fish.getAttributeBaseValue(Attributes.MAX_HEALTH), fish.getAttributeBaseValue(Attributes.MOVEMENT_SPEED),
                    defaultVariant, first, noEarly, second, touched, untouched, ally, data.getStringOr("Variant", ""),
                    custom.getStringOr("Variant", ""), released.getVariantString(), legacyFish.getVariantString());
            fish.discard();
            peer.discard();
            cod.discard();
            remoteCod.discard();
            source.sendSuccess(() -> Component.literal("jellyfish_fixture " + result), false);
            return 1;
        }));
    }

    private static boolean pulse(Jellyfish fish) {
        fish.setDeltaMovement(Vec3.ZERO);
        fish.aiStep();
        return Math.abs(fish.getDeltaMovement().x - 0.18) < 1.0e-8;
    }

    private static Jellyfish add(ServerLevel level, double x) {
        var fish = NaturalistEntityTypes.JELLYFISH.get().create(level, EntitySpawnReason.COMMAND);
        fish.snapTo(x, 92.0, 8.5, 0, 0);
        fish.setNoAi(true);
        fish.setNoGravity(true);
        level.addFreshEntity(fish);
        fish.baseTick();
        return fish;
    }
}
