package naturalist.fixture;

import com.google.gson.Gson;
import java.lang.reflect.Method;
import java.util.List;
import java.util.LinkedHashMap;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.network.chat.Component;
import net.minecraft.resources.Identifier;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.state.BlockState;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.PlayLevelSoundEvent;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class LandStepProbe {
    private static final String[] SPECIES = {
        "capybara", "turkey", "lizard", "lizard_tail", "komodo_dragon", "desert_scorpion", "jungle_scorpion", "rhino"
    };
    private List<?> heard;
    private boolean capturing;

    public LandStepProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
        NeoForge.EVENT_BUS.addListener(this::sound);
    }

    private void sound(PlayLevelSoundEvent.AtPosition event) {
        if (capturing && event.getSound() != null) {
            heard = List.of(event.getSound().value().location().toString(), event.getNewVolume(), event.getNewPitch());
        }
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("naturalist_fixture_land_step").executes(context -> {
            var source = context.getSource();
            var level = source.getLevel();
            var result = new LinkedHashMap<String, List<?>>();
            try {
                Method step = Entity.class.getDeclaredMethod("playStepSound", BlockPos.class, BlockState.class);
                step.setAccessible(true);
                for (String species : SPECIES) {
                    var type = BuiltInRegistries.ENTITY_TYPE.getValue(Identifier.parse("naturalist:" + species));
                    var mob = type.create(level, EntitySpawnReason.COMMAND);
                    heard = List.of();
                    capturing = true;
                    try {
                        step.invoke(mob, BlockPos.ZERO, Blocks.STONE.defaultBlockState());
                    } finally {
                        capturing = false;
                    }
                    result.put(species, heard);
                }
            } catch (ReflectiveOperationException ex) {
                throw new IllegalStateException(ex);
            }
            source.sendSuccess(() -> Component.literal("naturalist_fixture_land_step " + new Gson().toJson(result)), false);
            return 1;
        }));
    }
}
