package naturalist.fixture;

import com.google.gson.GsonBuilder;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import net.minecraft.commands.Commands;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.network.chat.Component;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.Mob;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class MobProfileProbe {
    public MobProfileProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("naturalist_mob_profile").executes(context -> {
            var source = context.getSource();
            var level = source.getLevel();
            var types = BuiltInRegistries.ENTITY_TYPE;
            var gson = new GsonBuilder().serializeNulls().create();
            var attributes = Map.of(
                "max_health", Attributes.MAX_HEALTH,
                "movement_speed", Attributes.MOVEMENT_SPEED,
                "follow_range", Attributes.FOLLOW_RANGE,
                "attack_damage", Attributes.ATTACK_DAMAGE,
                "attack_knockback", Attributes.ATTACK_KNOCKBACK,
                "knockback_resistance", Attributes.KNOCKBACK_RESISTANCE,
                "scale", Attributes.SCALE,
                "step_height", Attributes.STEP_HEIGHT
            );
            types.keySet().stream().filter(id -> id.getNamespace().equals("naturalist")).sorted().forEach(id -> {
                var type = types.getValue(id);
                if (!(type.create(level, EntitySpawnReason.COMMAND) instanceof Mob mob)) {
                    return;
                }
                var values = new LinkedHashMap<String, Double>();
                attributes.entrySet().stream().sorted(Map.Entry.comparingByKey()).forEach(entry -> {
                    var instance = mob.getAttribute(entry.getValue());
                    values.put(entry.getKey(), instance == null ? null : instance.getBaseValue());
                });
                var row = Map.of(
                    "id", id.toString(),
                    "category", type.getCategory().getName(),
                    "dimensions", List.of(mob.getBbWidth(), mob.getBbHeight(), mob.getEyeHeight()),
                    "attributes", values
                );
                source.sendSuccess(() -> Component.literal("naturalist_mob_profile " + gson.toJson(row)), false);
            });
            return 1;
        }));
    }
}
