package naturalist.fixture;

import com.google.gson.Gson;
import java.util.List;
import java.util.Map;
import net.minecraft.commands.Commands;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.util.ProblemReporter;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityType;
import net.minecraft.world.entity.Mob;
import net.minecraft.world.level.storage.TagValueInput;
import net.minecraft.world.level.storage.TagValueOutput;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class MobAgeProfileProbe {
    public MobAgeProfileProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("naturalist_mob_age_profile").executes(context -> {
            var source = context.getSource();
            var level = source.getLevel();
            var types = BuiltInRegistries.ENTITY_TYPE;
            var gson = new Gson();
            types.keySet().stream().filter(id -> id.getNamespace().equals("naturalist")).sorted().forEach(id -> {
                var type = types.getValue(id);
                var adult = ageProfile(level, type, 0);
                var young = ageProfile(level, type, -24000);
                if (adult == null || young == null) {
                    return;
                }
                var row = Map.of("id", id.toString(), "adult", adult, "juvenile", young);
                source.sendSuccess(() -> Component.literal("naturalist_mob_age_profile " + gson.toJson(row)), false);
            });
            return 1;
        }));
    }

    private static Map<String, Object> ageProfile(ServerLevel level, EntityType<?> type, int age) {
        if (!(type.create(level, EntitySpawnReason.COMMAND) instanceof Mob mob)) {
            return null;
        }
        var out = TagValueOutput.createWithContext(ProblemReporter.DISCARDING, level.registryAccess());
        mob.saveWithoutId(out);
        var tag = out.buildResult();
        tag.putInt("Age", age);
        tag.putBoolean("NoAI", true);
        mob.load(TagValueInput.create(ProblemReporter.DISCARDING, level.registryAccess(), tag));
        return Map.of("baby", mob.isBaby(),
                "dimensions", List.of(mob.getBbWidth(), mob.getBbHeight(), mob.getEyeHeight()));
    }
}
