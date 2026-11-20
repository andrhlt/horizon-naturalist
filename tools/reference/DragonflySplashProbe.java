package naturalist.fixture;

import com.google.gson.Gson;
import java.util.ArrayList;
import java.util.Map;
import net.minecraft.commands.Commands;
import net.minecraft.network.chat.Component;
import net.minecraft.world.level.gameevent.GameEvent;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.PlayLevelSoundEvent;
import net.neoforged.neoforge.event.RegisterCommandsEvent;
import net.neoforged.neoforge.event.VanillaGameEvent;

@Mod("naturalist_fixture")
public final class DragonflySplashProbe {
    private final ArrayList<String> gameEvents = new ArrayList<>();
    private final ArrayList<String> sounds = new ArrayList<>();

    public DragonflySplashProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
        NeoForge.EVENT_BUS.addListener(this::gameEvent);
        NeoForge.EVENT_BUS.addListener(this::sound);
    }

    private void gameEvent(VanillaGameEvent event) {
        var actor = event.getCause();
        if (event.getVanillaEvent().equals(GameEvent.SPLASH)
            && actor != null && actor.entityTags().contains("dragonfly_motion")) {
            gameEvents.add(actor.getType().getDescriptionId());
        }
    }

    private void sound(PlayLevelSoundEvent.AtPosition event) {
        var id = event.getSound().value().location().toString();
        var pos = event.getPosition();
        if (id.endsWith(".splash") && pos.x() > 10 && pos.x() < 14
            && pos.y() > 100 && pos.y() < 104 && pos.z() > 2 && pos.z() < 10) {
            sounds.add(id);
        }
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("dragonfly_splash_fixture")
            .then(Commands.literal("clear").executes(ctx -> {
                gameEvents.clear();
                sounds.clear();
                return 1;
            })).then(Commands.literal("report").executes(ctx -> {
                var report = new Gson().toJson(Map.of("events", gameEvents, "sounds", sounds));
                ctx.getSource().sendSuccess(() -> Component.literal("dragonfly_splash " + report), false);
                return 1;
            })));
    }
}
