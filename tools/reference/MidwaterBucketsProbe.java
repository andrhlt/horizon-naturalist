package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistRegistry;
import com.google.gson.Gson;
import java.util.List;
import java.util.TreeMap;
import net.minecraft.commands.Commands;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.network.chat.Component;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.entity.animal.fish.AbstractFish;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.phys.AABB;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.common.util.FakePlayerFactory;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class MidwaterBucketsProbe {
    private static final Gson GSON = new Gson();

    public MidwaterBucketsProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("midwater_buckets_fixture").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            var player = FakePlayerFactory.getMinecraft(level);
            player.snapTo(8.5, 92, 6.5, 0, 0);
            var items = List.of(NaturalistRegistry.CATFISH_BUCKET.get(),
                NaturalistRegistry.PIRANHA_BUCKET.get(), NaturalistRegistry.BLOBFISH_BUCKET.get());
            var tank = new AABB(4, 90, 7, 15, 96, 16);
            level.getEntitiesOfClass(AbstractFish.class, tank, fish -> true).forEach(AbstractFish::discard);
            var output = new TreeMap<String, String>();
            for (var item : items) {
                player.setItemInHand(InteractionHand.MAIN_HAND, new ItemStack(item));
                var action = item.use(level, player, InteractionHand.MAIN_HAND);
                var released = level.getEntitiesOfClass(AbstractFish.class, tank, AbstractFish::fromBucket);
                if (action.consumesAction() && released.size() == 1) {
                    var fish = released.getFirst();
                    output.put(BuiltInRegistries.ITEM.getKey(item).toString(),
                        BuiltInRegistries.ENTITY_TYPE.getKey(fish.getType()).toString());
                }
                for (var fish : released) {
                    fish.discard();
                }
            }
            source.sendSuccess(() -> Component.literal("midwater_buckets_fixture " + GSON.toJson(output)), false);
            return 1;
        }));
    }
}
