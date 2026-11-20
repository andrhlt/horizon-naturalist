package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.entity.animal.deer.Deer;
import net.minecraft.commands.Commands;
import net.minecraft.network.chat.Component;
import net.minecraft.resources.Identifier;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.Pose;
import net.minecraft.world.entity.ai.attributes.AttributeModifier;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class DeerScaleProbe {
    public DeerScaleProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("naturalist_fixture_deer_scale").executes(context -> {
            var source = context.getSource();
            var deer = NaturalistEntityTypes.DEER.get().create(source.getLevel(), EntitySpawnReason.COMMAND);
            if (deer == null) throw new IllegalStateException("Deer failed to register");
            var attribute = deer.getAttribute(Attributes.SCALE);
            if (attribute == null) throw new IllegalStateException("Scale missing");
            var baseline = dimensions(deer);
            attribute.setBaseValue(2.0D);
            var doubleScale = dimensions(deer);
            var id = Identifier.fromNamespaceAndPath("naturalist", "fixture_scale");
            attribute.addPermanentModifier(new AttributeModifier(id, 0.5D,
                    AttributeModifier.Operation.ADD_MULTIPLIED_TOTAL));
            var triple = dimensions(deer);
            deer.setAge(-24000);
            var fawn = dimensions(deer);
            attribute.removeModifier(id);
            attribute.setBaseValue(1.0D);
            var restored = dimensions(deer);
            var rows = "[" + String.join(",", baseline, doubleScale, triple, fawn, restored) + "]";
            source.sendSuccess(() -> Component.literal("naturalist_fixture_deer_scale " + rows), false);
            return 1;
        }));
    }

    private String dimensions(Deer deer) {
        var d = deer.getDimensions(Pose.STANDING);
        return "[" + deer.getScale() + "," + d.width() + "," + d.height() + "," + d.eyeHeight() + "]";
    }
}
