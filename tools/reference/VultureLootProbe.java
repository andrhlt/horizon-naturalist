package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import net.minecraft.commands.Commands;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EquipmentSlot;
import net.minecraft.world.entity.Mob;
import net.minecraft.world.entity.item.ItemEntity;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.minecraft.world.phys.AABB;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class VultureLootProbe {
    public VultureLootProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("vulture_loot_fixture").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            var area = new AABB(6, 92, 6, 11, 97, 11);
            level.getEntitiesOfClass(ItemEntity.class, area).forEach(Entity::discard);
            var vulture = NaturalistEntityTypes.VULTURE.get().create(level, EntitySpawnReason.COMMAND);
            if (vulture == null) throw new IllegalStateException("Missing vulture");
            vulture.snapTo(8.5, 93, 8.5, 0, 0);
            vulture.setItemSlot(EquipmentSlot.MAINHAND, new ItemStack(Items.STICK, 3));
            vulture.setItemSlot(EquipmentSlot.HEAD, new ItemStack(Items.IRON_HELMET));
            int xp;
            try {
                var method = Mob.class.getDeclaredMethod("getBaseExperienceReward", ServerLevel.class);
                method.setAccessible(true);
                xp = (int)method.invoke(vulture, level);
            } catch (ReflectiveOperationException error) {
                throw new IllegalStateException(error);
            }
            level.addFreshEntity(vulture);
            vulture.hurtServer(level, vulture.damageSources().generic(), 100f);
            var drops = level.getEntitiesOfClass(ItemEntity.class, area);
            int sticks = drops.stream().filter(e -> e.getItem().is(Items.STICK)).mapToInt(e -> e.getItem().getCount()).sum();
            int helmets = drops.stream().filter(e -> e.getItem().is(Items.IRON_HELMET)).mapToInt(e -> e.getItem().getCount()).sum();
            boolean cleared = vulture.getMainHandItem().isEmpty() && vulture.getItemBySlot(EquipmentSlot.HEAD).isEmpty();
            String report = ("{\"stick_count\":%d,\"helmet_count\":%d,\"only_equipment\":%s,"
                + "\"slots_emptied\":%s,\"base_experience\":%d}")
                .formatted(sticks, helmets, drops.size() == 2, cleared, xp);
            source.sendSuccess(() -> Component.literal("vulture_loot_fixture " + report), false);
            return 1;
        }));
    }
}
