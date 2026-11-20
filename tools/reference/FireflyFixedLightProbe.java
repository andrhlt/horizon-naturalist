package naturalist.fixture;

import com.crispytwig.naturalist.world.entity.animal.firefly.Firefly;
import com.google.gson.Gson;
import java.util.TreeMap;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.network.chat.Component;
import net.minecraft.world.level.Level;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class FireflyFixedLightProbe {
    public FireflyFixedLightProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("firefly_fixed_light_fixture").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getServer().getLevel(Level.NETHER);
            var output = new TreeMap<String, Object>();
            var groups = new TreeMap<String, Integer>();
            var totals = new TreeMap<String, Integer>();
            for (var entity : level.getAllEntities()) {
                if (entity instanceof Firefly firefly && firefly.entityTags().contains("fixed_light")) {
                    String group = firefly.entityTags().contains("bright_group") ? "bright" : "dark";
                    totals.merge(group, 1, Integer::sum);
                    if (firefly.isGlowing()) {
                        groups.merge(group, 1, Integer::sum);
                    }
                }
            }
            output.put("glowing", groups);
            output.put("total", totals);
            output.put("fixed", level.dimensionType().hasFixedTime());
            output.put("dark_outside", level.isDarkOutside());
            output.put("bright_raw", level.getMaxLocalRawBrightness(new BlockPos(7, 100, 7)));
            output.put("dark_raw", level.getMaxLocalRawBrightness(new BlockPos(23, 100, 7)));
            source.sendSuccess(() -> Component.literal("firefly_fixed_light " + new Gson().toJson(output)), false);
            return 1;
        }));
    }
}
