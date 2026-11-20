package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistRegistry;
import com.crispytwig.naturalist.world.level.block.SnailShellBlock;
import net.minecraft.commands.Commands;
import net.minecraft.core.component.DataComponents;
import net.minecraft.network.chat.Component;
import net.minecraft.world.item.DyeColor;
import net.minecraft.world.item.ItemStack;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class SnailShellDefaultProbe {
    public SnailShellDefaultProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("snail_shell_default_fixture").executes(ctx -> {
            var bare = new ItemStack(NaturalistRegistry.SNAIL_SHELL.get());
            var purple = SnailShellBlock.getShellStack(DyeColor.PURPLE);
            var white = SnailShellBlock.getShellStack(DyeColor.WHITE);
            var removed = bare.copy();
            removed.remove(DataComponents.CUSTOM_DATA);
            var result = """
                {"bare_is_brown":%s,"purple_is_purple":%s,"purple_is_not_brown":%s,"white_is_white":%s,"removal_stays_absent":%s}
                """.strip().formatted(color(bare) == 12, color(purple) == 10, color(purple) != 12,
                                       color(white) == 0, removed.get(DataComponents.CUSTOM_DATA) == null);
            ctx.getSource().sendSuccess(() -> Component.literal("snail_shell_default_fixture " + result), false);
            return 1;
        }));
    }

    private static int color(ItemStack stack) {
        var data = stack.get(DataComponents.CUSTOM_DATA);
        return data == null ? -1 : data.copyTag().getIntOr("Color", -1);
    }
}
