package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.registry.NaturalistRegistry;
import net.minecraft.commands.Commands;
import net.minecraft.core.component.DataComponents;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.entity.item.ItemEntity;
import net.minecraft.world.item.Items;
import net.minecraft.world.item.component.CustomData;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class CatfishProbe {
    public CatfishProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("catfish_fixture").executes(ctx -> {
            var source = ctx.getSource();
            ServerLevel level = source.getLevel();
            var catfish = NaturalistEntityTypes.CATFISH.get().create(level, EntitySpawnReason.COMMAND);
            var bass = NaturalistEntityTypes.BASS.get().create(level, EntitySpawnReason.COMMAND);
            catfish.snapTo(8.5, 92.0, 8.5, 0, 0);
            bass.snapTo(8.8, 92.0, 8.5, 0, 0);
            catfish.setNoAi(true);
            bass.setNoAi(true);
            level.addFreshEntity(catfish);
            level.addFreshEntity(bass);
            boolean bite = catfish.doHurtTarget(level, bass);
            boolean aliveAfterBite = bass.isAlive();
            int biteCooldown = catfish.getHuntingCooldown();
            for (int tick = 0; tick < 7; tick++) {
                catfish.aiStep();
            }
            boolean aliveAfterSeven = bass.isAlive();
            catfish.aiStep();
            boolean goneAfterEight = bass.isRemoved();
            boolean boneMeal = !level.getEntitiesOfClass(ItemEntity.class, catfish.getBoundingBox().inflate(4),
                item -> item.getItem().is(Items.BONE_MEAL)).isEmpty();
            var bucket = catfish.getBucketItemStack();
            catfish.saveToBucketTag(bucket);
            var bucketData = bucket.getOrDefault(DataComponents.BUCKET_ENTITY_DATA, CustomData.EMPTY).copyTag();
            String result = """
                {"max_health":%s,"attack_damage":%s,"movement_speed":%s,"variant":"%s",\
                "bite_landed":%s,"prey_alive_after_bite":%s,"cooldown_after_bite":%d,\
                "prey_alive_after_seven":%s,"prey_gone_after_eight":%s,"bone_meal":%s,\
                "cooldown_after_eight":%d,"bucket_has_variant":%s,"bucket_has_cooldown":%s}
                """.strip().formatted(catfish.getAttributeBaseValue(Attributes.MAX_HEALTH),
                    catfish.getAttributeBaseValue(Attributes.ATTACK_DAMAGE),
                    catfish.getAttributeBaseValue(Attributes.MOVEMENT_SPEED), catfish.getVariantString(),
                    bite, aliveAfterBite, biteCooldown, aliveAfterSeven, goneAfterEight, boneMeal,
                    catfish.getHuntingCooldown(), bucketData.contains("Variant"), bucketData.contains("HuntingCooldown"));
            source.sendSuccess(() -> Component.literal("catfish_fixture " + result), false);
            return 1;
        }));
    }
}
