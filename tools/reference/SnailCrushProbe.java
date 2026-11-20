package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.entity.animal.snail.Snail;
import net.minecraft.commands.Commands;
import net.minecraft.core.registries.Registries;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EquipmentSlot;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.minecraft.world.item.enchantment.Enchantments;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.common.util.FakePlayer;
import net.neoforged.neoforge.common.util.FakePlayerFactory;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class SnailCrushProbe {
    public SnailCrushProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("snail_crush_fixture").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            var player = FakePlayerFactory.getMinecraft(level);
            level.addNewPlayer(player);
            try {
                var boots = new ItemStack(Items.IRON_BOOTS);
                var featherFalling = level.registryAccess().lookupOrThrow(Registries.ENCHANTMENT)
                        .getOrThrow(Enchantments.FEATHER_FALLING);
                boots.enchant(featherFalling, 1);
                float airborne = check(level, player, 2.5, ItemStack.EMPTY, false, false, false, false);
                float feather = check(level, player, 4.5, boots, false, false, false, false);
                float grounded = check(level, player, 6.5, ItemStack.EMPTY, true, false, false, false);
                float named = check(level, player, 10.5, ItemStack.EMPTY, false, true, false, false);
                float persistent = check(level, player, 12.5, ItemStack.EMPTY, false, false, true, false);
                float fromHand = check(level, player, 14.5, ItemStack.EMPTY, false, false, false, true);
                var result = """
                    {"airborne_health":%s,"feather_health":%s,"grounded_health":%s,"named_health":%s,"persistent_health":%s,"from_hand_health":%s}
                    """.strip().formatted(airborne, feather, grounded, named, persistent, fromHand);
                source.sendSuccess(() -> Component.literal("snail_crush_fixture " + result), false);
            } finally {
                level.removePlayerImmediately(player, Entity.RemovalReason.DISCARDED);
            }
            return 1;
        }));
    }

    private static float check(ServerLevel level, FakePlayer player, double x, ItemStack feet,
                               boolean grounded, boolean named, boolean persistent, boolean fromHand) {
        Snail snail = NaturalistEntityTypes.SNAIL.get().create(level, EntitySpawnReason.COMMAND);
        if (snail == null) {
            throw new IllegalStateException("Could not create Snail");
        }
        snail.snapTo(x, 93, 8.5, 0, 0);
        snail.setNoAi(true);
        snail.setOnGround(true);
        if (named) {
            snail.setCustomName(Component.literal("Protected"));
        }
        if (persistent) {
            snail.setPersistenceRequired();
        }
        snail.setFromHand(fromHand);
        level.addFreshEntity(snail);
        player.setItemSlot(EquipmentSlot.FEET, feet);
        player.snapTo(x, 93.35, 8.5, 0, 0);
        player.setOnGround(grounded);
        if (level.getEntitiesOfClass(FakePlayer.class, snail.getBoundingBox().inflate(0.2, 0.5, 0.2))
                .stream().noneMatch(p -> p == player)) {
            throw new IllegalStateException("Test player is absent from the impact bounds");
        }
        snail.aiStep();
        float health = snail.getHealth();
        snail.discard();
        return health;
    }
}
