package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.registry.NaturalistRegistry;
import com.crispytwig.naturalist.world.entity.animal.hedgehog.Hedgehog;
import net.minecraft.commands.Commands;
import net.minecraft.core.component.DataComponents;
import net.minecraft.core.registries.Registries;
import net.minecraft.network.chat.Component;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.InteractionResult;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityTypes;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.component.CustomData;
import net.minecraft.world.item.enchantment.Enchantments;
import net.minecraft.world.level.GameType;
import net.minecraft.world.phys.AABB;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.common.util.FakePlayerFactory;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class HedgehogThrowProbe {
    public HedgehogThrowProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("hedgehog_throw_fixture").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            var player = FakePlayerFactory.getMinecraft(level);
            level.addNewPlayer(player);
            player.setGameMode(GameType.SURVIVAL);
            player.getInventory().clearContent();
            player.snapTo(8.5, 93.0, 6.5, 0.0F, 0.0F);
            try {
                var ownerPet = NaturalistEntityTypes.HEDGEHOG.get().create(level, EntitySpawnReason.COMMAND);
                ownerPet.tame(player);
                var held = ownerPet.getCaughtItemStack();
                ownerPet.saveToHandTag(held);
                var registry = level.registryAccess().lookupOrThrow(Registries.ENCHANTMENT);
                held.enchant(registry.getOrThrow(Enchantments.THORNS), 2);
                held.enchant(registry.getOrThrow(Enchantments.LOYALTY), 3);
                held.enchant(registry.getOrThrow(Enchantments.FLAME), 1);
                var item = NaturalistRegistry.HEDGEHOG.get();
                player.setItemInHand(InteractionHand.MAIN_HAND, held);
                var pig = EntityTypes.PIG.create(level, EntitySpawnReason.COMMAND);
                pig.snapTo(8.5, 93.9, 8.15, 0, 0);
                pig.setNoAi(true);
                pig.setNoGravity(true);
                level.addFreshEntity(pig);

                var use = item.use(level, player, InteractionHand.MAIN_HAND);
                var thrown = level.getEntitiesOfClass(Hedgehog.class,
                        new AABB(7, 93, 6, 10, 97, 9), Hedgehog::isRolling).getFirst();
                var at = thrown.position();
                var velocity = thrown.getDeltaMovement();
                boolean transientRoll = thrown.isRolling() && thrown.fromHand();
                boolean consumed = held.isEmpty() && use == InteractionResult.SUCCESS;
                for (int i = 0; i < 120 && !thrown.isRemoved(); i++) {
                    level.tickNonPassenger(thrown);
                }
                ItemStack returned = ItemStack.EMPTY;
                var inventory = player.getInventory();
                for (int i = 0; i < inventory.getContainerSize(); i++) {
                    var stack = inventory.getItem(i);
                    if (stack.is(item)) returned = stack;
                }
                var data = returned.getOrDefault(DataComponents.CUSTOM_DATA, CustomData.EMPTY).copyTag();
                var projectile = """
                    {"roll":%s,"consumed":%s,"launch":[%s,%s,%s],"motion":[%s,%s,%s],"pig_health":%s,"pig_fire":%s,"return":%s,"returned_health":%s,"returned_owner":%s,"enchants":%s}
                    """.strip().formatted(transientRoll, consumed, at.x, at.y, at.z, velocity.x, velocity.y, velocity.z,
                        pig.getHealth(), pig.getRemainingFireTicks(), thrown.isRemoved() && !returned.isEmpty(),
                        data.getFloatOr("Health", -1), data.contains("Owner"), returned.isEnchanted());
                source.sendSuccess(() -> Component.literal("hedgehog_throw_fixture " + projectile), false);
                return 1;
            } finally {
                level.removePlayerImmediately(player, Entity.RemovalReason.DISCARDED);
            }
        }));
    }
}
