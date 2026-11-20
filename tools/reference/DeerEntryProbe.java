package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.entity.animal.deer.Deer;
import com.google.gson.GsonBuilder;
import com.mojang.brigadier.arguments.LongArgumentType;
import java.util.LinkedHashMap;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.network.chat.Component;
import net.minecraft.resources.Identifier;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.phys.AABB;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class DeerEntryProbe {
    public DeerEntryProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("naturalist_fixture_deer_entry")
                .then(Commands.literal("seed")
                        .then(Commands.argument("seed", LongArgumentType.longArg()).executes(ctx -> {
                            var level = ctx.getSource().getLevel();
                            level.getRandom().setSeed(LongArgumentType.getLong(ctx, "seed"));
                            ctx.getSource().sendSuccess(() -> Component.literal("Deer entry seeded"), false);
                            return 1;
                        })))
                .then(Commands.literal("egg").executes(ctx -> {
                    var deer = NaturalistEntityTypes.DEER.get().spawn(ctx.getSource().getLevel(),
                            new BlockPos(8, 100, 8), EntitySpawnReason.SPAWN_ITEM_USE);
                    if (deer != null) deer.addTag("entry_fixture");
                    return deer != null ? 1 : 0;
                }))
                .then(Commands.literal("observe").executes(ctx -> {
                    var source = ctx.getSource();
                    var level = source.getLevel();
                    var region = new AABB(7, 94, 7, 10, 105, 10);
                    var deer = level.getEntitiesOfClass(Deer.class, region).stream().findFirst().orElse(null);
                    if (deer == null) {
                        source.sendFailure(Component.literal("Deer entry has no actor"));
                        return 0;
                    }
                    var modifier = deer.getAttribute(Attributes.FOLLOW_RANGE)
                            .getModifier(Identifier.withDefaultNamespace("random_spawn_bonus"));
                    var row = new LinkedHashMap<String, Object>();
                    row.put("variant", deer.getVariantString());
                    row.put("follow_bonus", modifier == null ? null : modifier.amount());
                    row.put("left_handed", deer.isLeftHanded());
                    row.put("age", deer.getAge());
                    row.put("yaw", deer.getYRot());
                    row.put("next", level.getRandom().nextLong());
                    deer.discard();
                    var text = new GsonBuilder().serializeNulls().create().toJson(row);
                    source.sendSuccess(() -> Component.literal("Naturalist Deer entry " + text), false);
                    return 1;
                })));
    }
}
