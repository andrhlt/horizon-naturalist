package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.google.gson.Gson;
import java.util.LinkedHashMap;
import java.util.Map;
import net.minecraft.commands.Commands;
import net.minecraft.network.chat.Component;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityType;
import net.minecraft.world.entity.LivingEntity;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.PlayLevelSoundEvent;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class VoiceLevelsProbe {
    private Map<String, Map<String, Object>> recording;

    public VoiceLevelsProbe() {
        NeoForge.EVENT_BUS.addListener(this::sound);
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void sound(PlayLevelSoundEvent.AtPosition event) {
        if (recording == null || event.getSound() == null) {
            return;
        }
        String id = event.getSound().value().location().toString();
        if (id.startsWith("naturalist:entity.") && id.endsWith(".hurt")) {
            recording.put(id, Map.of("volume", event.getOriginalVolume(), "source", event.getSource().getName()));
        }
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("naturalist_fixture_voice_levels")
                .executes(context -> {
                    var source = context.getSource();
                    var level = source.getLevel();
                    var result = new LinkedHashMap<String, Map<String, Object>>();
                    Map<String, EntityType<? extends LivingEntity>> types = Map.of(
                            "bear", NaturalistEntityTypes.BEAR.get(),
                            "black_bear", NaturalistEntityTypes.BLACK_BEAR.get(),
                            "boar", NaturalistEntityTypes.BOAR.get(),
                            "turkey", NaturalistEntityTypes.TURKEY.get(),
                            "capybara", NaturalistEntityTypes.CAPYBARA.get(),
                            "desert_scorpion", NaturalistEntityTypes.DESERT_SCORPION.get(),
                            "jungle_scorpion", NaturalistEntityTypes.JUNGLE_SCORPION.get());
                    for (var sample : types.entrySet()) {
                        var sound = new LinkedHashMap<String, Map<String, Object>>();
                        var animal = sample.getValue().create(level, EntitySpawnReason.COMMAND);
                        animal.snapTo(8.5, 101, 8.5, 0, 0);
                        recording = sound;
                        try {
                            animal.hurtServer(level, level.damageSources().generic(), 1);
                        } finally {
                            recording = null;
                        }
                        if (sound.size() != 1) {
                            throw new IllegalStateException("Wrong sound count for " + sample.getKey() + ": " + sound);
                        }
                        result.put(sample.getKey(), sound.values().iterator().next());
                    }
                    source.sendSuccess(() -> Component.literal("naturalist_voice_levels " + new Gson().toJson(result)), false);
                    return 1;
                }));
    }
}
