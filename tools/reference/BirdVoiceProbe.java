package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.entity.animal.bird.Bird;
import java.util.List;
import net.minecraft.commands.Commands;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.network.chat.Component;
import net.minecraft.sounds.SoundEvent;
import net.minecraft.world.entity.EntitySpawnReason;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class BirdVoiceProbe {
    public BirdVoiceProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("bird_fixture_voice").executes(ctx -> {
            var source = ctx.getSource();
            var bird = NaturalistEntityTypes.BIRD.get().create(source.getLevel(), EntitySpawnReason.COMMAND);
            if (bird == null) throw new IllegalStateException("Bird type was not declared");
            try {
                var method = Bird.class.getDeclaredMethod("getAmbientSound");
                method.setAccessible(true);
                var list = new java.util.ArrayList<String>();
                for (var variant : List.of("blue_jay", "stellers_jay", "northern_cardinal", "carolina_chickadee",
                                           "tufted_titmouse", "red_winged_blackbird", "white_throated_sparrow",
                                           "unknown_fallback")) {
                    bird.setVariantString("naturalist:" + variant);
                    var sound = (SoundEvent) method.invoke(bird);
                    var id = sound == null ? null : BuiltInRegistries.SOUND_EVENT.getKey(sound);
                    list.add("\"" + variant + "\":" + (id == null ? "null" : "\"" + id + "\""));
                }
                var output = "bird_fixture_voice {" + String.join(",", list) + "}";
                source.sendSuccess(() -> Component.literal(output), false);
                return 1;
            } catch (ReflectiveOperationException cause) {
                throw new IllegalStateException(cause);
            }
        }));
    }
}
