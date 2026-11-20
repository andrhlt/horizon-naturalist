package naturalist.fixture;

import com.crispytwig.naturalist.world.entity.animal.rat.Rat;
import com.google.gson.Gson;
import java.util.List;
import net.minecraft.commands.Commands;
import net.minecraft.network.chat.Component;
import net.minecraft.commands.arguments.EntityArgument;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class RatSleepProbe {
    public RatSleepProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("rat_sleep_probe")
                .then(Commands.argument("rat", EntityArgument.entity()).executes(context -> {
            var source = context.getSource();
            var rat = (Rat) EntityArgument.getEntity(context, "rat");
            var result = List.of(rat.canSleep(), rat.isSleeping(), rat.onGround());
            source.sendSuccess(() -> Component.literal("rat_sleep_probe " + new Gson().toJson(result)), false);
            return 1;
        })));
    }
}
