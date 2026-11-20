package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.entity.animal.giantisopod.GiantIsopod;
import com.google.gson.Gson;
import com.mojang.authlib.GameProfile;
import java.util.LinkedHashMap;
import java.util.UUID;
import net.minecraft.commands.Commands;
import net.minecraft.core.component.DataComponents;
import net.minecraft.network.Connection;
import net.minecraft.network.chat.Component;
import net.minecraft.network.protocol.PacketFlow;
import net.minecraft.server.level.ClientInformation;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.server.network.CommonListenerCookie;
import net.minecraft.server.network.ServerGamePacketListenerImpl;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.item.Items;
import net.minecraft.world.effect.MobEffectInstance;
import net.minecraft.world.effect.MobEffects;
import net.minecraft.core.BlockPos;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.level.GameType;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.item.component.CustomData;
import net.minecraft.world.phys.Vec3;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class GiantIsopodProbe {
    private static final Gson GSON = new Gson();

    public GiantIsopodProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("giant_isopod_fixture").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            try {
                var player = spawnPlayer(level);
                var adult = add(level, 8.5);
                var baby = add(level, 8.7);
                baby.setBaby(true);
                var outside = add(level, 13.5);
                var result = new LinkedHashMap<String, Object>();
                result.put("max_health", adult.getAttributeBaseValue(Attributes.MAX_HEALTH));
                result.put("speed", adult.getAttributeBaseValue(Attributes.MOVEMENT_SPEED));
                result.put("variant", adult.getVariantString());
                result.put("breathes_water", adult.canBreatheUnderwater());
                result.put("near_hides", adult.canHide());
                result.put("baby_hides", baby.canHide());
                result.put("outside_hides", outside.canHide());
                result.put("cluster", adult.getMaxSpawnClusterSize());
                adult.hurtServer(level, level.damageSources().generic(), 4.0F);
                outside.hurtServer(level, level.damageSources().generic(), 4.0F);
                result.put("near_health", adult.getHealth());
                result.put("outside_health", outside.getHealth());
                adult.setDeltaMovement(Vec3.ZERO);
                outside.setDeltaMovement(Vec3.ZERO);
                adult.knockback(0.4, 1.0, 0.0, level.damageSources().generic(), 0.0F, false);
                outside.knockback(0.4, 1.0, 0.0, level.damageSources().generic(), 0.0F, false);
                result.put("near_knockback", Math.round(Math.abs(adult.getDeltaMovement().x) * 1000));
                result.put("outside_knockback", Math.round(Math.abs(outside.getDeltaMovement().x) * 1000));
                player.snapTo(20.0, 92, 8.5, 0, 0);
                for (int tick = 0; tick < 9; tick++) adult.aiStep();
                result.put("holds_after_9", adult.canHide());
                adult.aiStep();
                result.put("holds_after_10", adult.canHide());

                adult.setVariantString("naturalist:blue");
                var stack = adult.getBucketItemStack();
                adult.saveToBucketTag(stack);
                var entity = stack.getOrDefault(DataComponents.BUCKET_ENTITY_DATA, CustomData.EMPTY).copyTag();
                var custom = stack.getOrDefault(DataComponents.CUSTOM_DATA, CustomData.EMPTY).copyTag();
                result.put("bucket_variant", entity.getStringOr("Variant", ""));
                result.put("custom_variant", custom.getStringOr("Variant", ""));
                var released = NaturalistEntityTypes.GIANT_ISOPOD.get().create(level, EntitySpawnReason.BUCKET);
                released.loadFromBucketTag(entity);
                result.put("released_variant", released.getVariantString());
                entity.putInt("Variant", -2);
                released.loadFromBucketTag(entity);
                result.put("legacy_variant", released.getVariantString());
                result.put("kelp_food", adult.isFood(new ItemStack(Items.KELP)));
                result.put("cod_food", adult.isFood(new ItemStack(Items.COD)));

                player.snapTo(8.5, 92, 6.0, 0, 0);
                player.setGameMode(GameType.CREATIVE);
                var creative = add(level, 8.5);
                result.put("creative_hides", creative.canHide());
                creative.discard();
                player.setGameMode(GameType.SURVIVAL);

                player.addEffect(new MobEffectInstance(MobEffects.INVISIBILITY, 200));
                player.setInvisible(true);
                var hidden = add(level, 8.5);
                result.put("invisible_player_hides", hidden.canHide());
                hidden.discard();
                player.removeEffect(MobEffects.INVISIBILITY);
                player.setInvisible(false);

                var wall = new BlockPos(8, 92, 7);
                level.setBlockAndUpdate(wall, Blocks.STONE.defaultBlockState());
                var occluded = add(level, 8.5);
                result.put("wall_hides", occluded.canHide());
                occluded.discard();
                level.setBlockAndUpdate(wall, Blocks.AIR.defaultBlockState());

                adult.discard();
                outside.discard();
                baby.discard();
                player.discard();
                source.sendSuccess(() -> Component.literal("giant_isopod_fixture " + GSON.toJson(result)), false);
            } catch (Exception e) {
                e.printStackTrace();
                source.sendSuccess(() -> Component.literal("giant_isopod_fixture_error " + e), false);
            }
            return 1;
        }));
    }

    private static GiantIsopod add(ServerLevel level, double x) {
        var mob = NaturalistEntityTypes.GIANT_ISOPOD.get().create(level, EntitySpawnReason.COMMAND);
        mob.snapTo(x, 92, 8.5, 0, 0);
        mob.setNoAi(true);
        mob.setNoGravity(true);
        level.addFreshEntity(mob);
        return mob;
    }

    private static ServerPlayer spawnPlayer(ServerLevel level) {
        var profile = new GameProfile(UUID.randomUUID(), "IsopodFixture");
        var player = new ServerPlayer(level.getServer(), level, profile, ClientInformation.createDefault());
        var connection = new Connection(PacketFlow.SERVERBOUND);
        player.connection = new ServerGamePacketListenerImpl(level.getServer(), connection, player,
            CommonListenerCookie.createInitial(profile, false));
        player.snapTo(8.5, 92, 7.5, 0, 0);
        level.addNewPlayer(player);
        return player;
    }
}
