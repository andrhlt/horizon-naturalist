package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.entity.animal.fish.Blobfish;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.core.component.DataComponents;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.item.component.CustomData;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class BlobfishProbe {
    public BlobfishProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("blobfish_fixture").executes(ctx -> {
            var source = ctx.getSource();
            ServerLevel level = source.getLevel();
            var type = NaturalistEntityTypes.BLOBFISH.get();
            var fish = type.create(level, EntitySpawnReason.COMMAND);
            fish.setNoAi(true);
            fish.setGray(false);
            fish.snapTo(8.5, 26.0, 8.5, 0, 0);
            level.addFreshEntity(fish);
            fish.baseTick();
            tick(fish, 99);
            boolean pinkAfterNinetyNine = !fish.isGray() && fish.isConverting();
            tick(fish, 1);
            boolean grayAfterOneHundred = fish.isGray() && !fish.isConverting();
            move(fish, 54.0);
            tick(fish, 40);
            boolean ascentConverts = fish.isConverting() && fish.isGray();
            move(fish, 26.0);
            tick(fish, 1);
            boolean canceled = fish.isGray() && !fish.isConverting();
            move(fish, 54.0);
            tick(fish, 99);
            boolean grayAfterNinetyNine = fish.isGray() && fish.isConverting();
            tick(fish, 1);
            boolean pinkAfterOneHundred = !fish.isGray() && !fish.isConverting();

            fish.setGray(true);
            var bucket = fish.getBucketItemStack();
            fish.saveToBucketTag(bucket);
            var data = bucket.getOrDefault(DataComponents.BUCKET_ENTITY_DATA, CustomData.EMPTY).copyTag();
            var released = type.create(level, EntitySpawnReason.BUCKET);
            released.loadFromBucketTag(data);
            int threshold = level.getSeaLevel() - 33;
            boolean atThreshold = Blobfish.checkBlobfishSpawnRules(type, level, EntitySpawnReason.NATURAL,
                new BlockPos(8, threshold, 8), level.getRandom());
            boolean aboveThreshold = Blobfish.checkBlobfishSpawnRules(type, level, EntitySpawnReason.NATURAL,
                new BlockPos(8, threshold + 1, 8), level.getRandom());
            String result = """
                {"max_health":%s,"attack_damage":%s,"movement_speed":%s,\
                "pink_after_99":%s,"gray_after_100":%s,"ascent_converts":%s,"canceled":%s,\
                "gray_after_99":%s,"pink_after_100":%s,"bucket_variant":"%s","bucket_has_gray":%s,\
                "released_is_pink":%s,"at_threshold":%s,"above_threshold":%s}
                """.strip().formatted(fish.getAttributeBaseValue(Attributes.MAX_HEALTH),
                    fish.getAttributeBaseValue(Attributes.ATTACK_DAMAGE), fish.getAttributeBaseValue(Attributes.MOVEMENT_SPEED),
                    pinkAfterNinetyNine, grayAfterOneHundred, ascentConverts, canceled, grayAfterNinetyNine,
                    pinkAfterOneHundred, data.getStringOr("Variant", ""), data.contains("Gray"),
                    !released.isGray(), atThreshold, aboveThreshold);
            fish.discard();
            source.sendSuccess(() -> Component.literal("blobfish_fixture " + result), false);
            return 1;
        }));
    }

    private static void tick(Blobfish fish, int count) {
        for (int step = 0; step < count; step++) {
            fish.aiStep();
        }
    }

    private static void move(Blobfish fish, double y) {
        fish.snapTo(8.5, y, 8.5, 0, 0);
        fish.baseTick();
    }
}
