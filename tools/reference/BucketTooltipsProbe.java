package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistRegistry;
import com.crispytwig.naturalist.world.item.NaturalistBucketItem;
import com.google.gson.Gson;
import java.util.ArrayList;
import java.util.List;
import java.util.TreeMap;
import net.minecraft.commands.Commands;
import net.minecraft.core.component.DataComponents;
import net.minecraft.nbt.CompoundTag;
import net.minecraft.nbt.IntTag;
import net.minecraft.nbt.StringTag;
import net.minecraft.nbt.Tag;
import net.minecraft.network.chat.Component;
import net.minecraft.network.chat.contents.TranslatableContents;
import net.minecraft.world.item.Item;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.component.CustomData;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class BucketTooltipsProbe {
    private static final Gson GSON = new Gson();

    public BucketTooltipsProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    record Scenario(String name, NaturalistBucketItem item, Tag variant, boolean custom) {}

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("bucket_tooltips_fixture").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            var inputs = List.of(
                new Scenario("small_bass", NaturalistRegistry.BASS_BUCKET.get(), StringTag.valueOf("naturalist:bass"), true),
                new Scenario("legacy_large_bass", NaturalistRegistry.BASS_BUCKET.get(), IntTag.valueOf(-1), true),
                new Scenario("legacy_blue_jellyfish", NaturalistRegistry.JELLYFISH_BUCKET.get(), IntTag.valueOf(-2), true),
                new Scenario("legacy_mobula_ray", NaturalistRegistry.RAY_BUCKET.get(), IntTag.valueOf(-2), true),
                new Scenario("glow_anglerfish", NaturalistRegistry.ANGLERFISH_BUCKET.get(), StringTag.valueOf("naturalist:glow"), true),
                new Scenario("entity_only_ray", NaturalistRegistry.RAY_BUCKET.get(), StringTag.valueOf("naturalist:stingray"), false),
                new Scenario("invalid_ray", NaturalistRegistry.RAY_BUCKET.get(), StringTag.valueOf("Not an identifier"), true),
                new Scenario("plain_catfish", NaturalistRegistry.CATFISH_BUCKET.get(), StringTag.valueOf("naturalist:catfish"), true)
            );
            var result = new TreeMap<String, List<String>>();
            for (var input : inputs) {
                var stack = new ItemStack(input.item());
                var tag = new CompoundTag();
                tag.put("Variant", input.variant());
                stack.set(input.custom() ? DataComponents.CUSTOM_DATA : DataComponents.BUCKET_ENTITY_DATA, CustomData.of(tag));
                var lines = new ArrayList<Component>();
                input.item().appendHoverText(stack, Item.TooltipContext.of(level), null, lines::add, null);
                result.put(input.name(), lines.stream().map(line ->
                    ((TranslatableContents) line.getContents()).getKey() + " " + line.getStyle().getColor().getValue()).toList());
            }
            source.sendSuccess(() -> Component.literal("bucket_tooltips_fixture " + GSON.toJson(result)), false);
            return 1;
        }));
    }
}
