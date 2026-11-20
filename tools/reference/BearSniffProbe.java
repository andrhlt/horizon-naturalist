package naturalist.fixture;

import net.minecraft.commands.Commands;
import net.minecraft.network.chat.Component;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.PlayLevelSoundEvent;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class BearSniffProbe {
    private int bearSounds;
    private int blackBearSounds;

    public BearSniffProbe() {
        NeoForge.EVENT_BUS.addListener(this::sound);
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void sound(PlayLevelSoundEvent.AtPosition event) {
        if (event.getSound() == null) {
            return;
        }
        switch (event.getSound().value().location().toString()) {
            case "naturalist:entity.bear.sniff" -> bearSounds++;
            case "naturalist:entity.black_bear.sniff" -> blackBearSounds++;
        }
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("naturalist_fixture_bear_sniff")
                .executes(ctx -> {
                    ctx.getSource().sendSuccess(() -> Component.literal(
                            "naturalist_bear_sniff " + bearSounds + " " + blackBearSounds), false);
                    return 1;
                }));
    }
}
