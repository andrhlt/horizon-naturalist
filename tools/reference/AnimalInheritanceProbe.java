package naturalist.fixture;

import java.util.ArrayList;
import net.minecraft.commands.Commands;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.network.chat.Component;
import net.minecraft.resources.Identifier;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.animal.Animal;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class AnimalInheritanceProbe {
    public AnimalInheritanceProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("animal_inheritance_fixture").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            var rows = new ArrayList<String>();
            for (String name : """
                anglerfish bass bear bird black_bear blobfish boar butterfly capybara caterpillar
                catfish clam crab deer desert_scorpion dragonfly duck firefly giant_isopod jellyfish
                jungle_scorpion komodo_dragon lizard lizard_tail piranha ray rhino snail starfish turkey vulture
                """.strip().split("\\s+")) {
                var key = Identifier.fromNamespaceAndPath("naturalist", name);
                var type = BuiltInRegistries.ENTITY_TYPE.getValue(key);
                var mob = type.create(level, EntitySpawnReason.COMMAND);
                if (mob == null) throw new IllegalStateException("Missing " + name);
                rows.add(Character.toString(34) + key + Character.toString(34) + ":" + (mob instanceof Animal));
            }
            var json = "{" + String.join(",", rows) + "}";
            source.sendSuccess(() -> Component.literal("animal_inheritance_fixture " + json), false);
            return 1;
        }));
    }
}
