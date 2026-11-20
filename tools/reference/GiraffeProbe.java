package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.entity.animal.giraffe.Giraffe;
import com.google.gson.Gson;
import com.mojang.authlib.GameProfile;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.UUID;
import net.minecraft.commands.Commands;
import net.minecraft.network.Connection;
import net.minecraft.network.chat.Component;
import net.minecraft.network.protocol.PacketFlow;
import net.minecraft.server.level.ClientInformation;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.server.network.CommonListenerCookie;
import net.minecraft.server.network.ServerGamePacketListenerImpl;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.minecraft.world.level.GameType;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class GiraffeProbe {
    public GiraffeProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("naturalist_giraffe_fixture").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            var owner = player(level, "GiraffeOwner");
            var visitor = player(level, "GiraffeVisitor");
            var adult = giraffe(level, 8.5);
            var foal = giraffe(level, 10.5);
            try {
                var fields = new LinkedHashMap<String, Object>();
                fields.put("default_variant", adult.getVariantString());
                fields.put("speed", adult.getAttributeBaseValue(Attributes.MOVEMENT_SPEED));
                fields.put("health", adult.getMaxHealth());
                fields.put("step_height", adult.getAttributeBaseValue(Attributes.STEP_HEIGHT));
                fields.put("head_turn_limit", adult.getMaxHeadYRot());
                fields.put("adult_size", dimensions(adult));
                fields.put("can_climb", adult.onClimbable());
                fields.put("food", List.of(Items.APPLE, Items.GOLDEN_APPLE, Items.HAY_BLOCK,
                    Items.ENCHANTED_GOLDEN_APPLE, Items.WHEAT, Items.CARROT).stream()
                    .map(item -> adult.isFood(new ItemStack(item))).toList());

                foal.setAge(-2000);
                fields.put("baby_size", dimensions(foal));
                var adultFood = new ItemStack(Items.APPLE, 3);
                owner.setItemInHand(InteractionHand.MAIN_HAND, adultFood);
                adult.mobInteract(owner, InteractionHand.MAIN_HAND);
                fields.put("default_adult_food_remaining", adultFood.getCount());
                fields.put("adult_love", adult.getInLoveTime());
                fields.put("adult_tames_on_food", adult.isTame());

                var foalFood = new ItemStack(Items.APPLE, 32);
                owner.setItemInHand(InteractionHand.MAIN_HAND, foalFood);
                int tries = 0;
                while (!foal.isTame() && tries < 32) {
                    foal.mobInteract(owner, InteractionHand.MAIN_HAND);
                    tries++;
                }
                fields.put("baby_taming", foal.isTame() && foal.isOwnedBy(owner));
                fields.put("baby_food_cost_per_attempt", foalFood.getCount() == 32 - tries);
                fields.put("baby_age_after_tame", foal.getAge());
                owner.setItemInHand(InteractionHand.MAIN_HAND, ItemStack.EMPTY);
                foal.mobInteract(owner, InteractionHand.MAIN_HAND);
                fields.put("baby_empty_hand_mount", owner.getVehicle() == foal);

                adult.tame(owner);
                adult.setHealth(20);
                var treat = new ItemStack(Items.HAY_BLOCK, 3);
                owner.setItemInHand(InteractionHand.MAIN_HAND, treat);
                adult.mobInteract(owner, InteractionHand.MAIN_HAND);
                fields.put("owned_food_remaining", treat.getCount());
                fields.put("owned_heal", adult.getHealth());
                var visitorTreat = new ItemStack(Items.HAY_BLOCK, 3);
                visitor.setItemInHand(InteractionHand.MAIN_HAND, visitorTreat);
                adult.mobInteract(visitor, InteractionHand.MAIN_HAND);
                fields.put("visitor_does_not_heal", adult.getHealth() == 24);
                fields.put("visitor_food_remaining", visitorTreat.getCount());

                visitor.setItemInHand(InteractionHand.MAIN_HAND, ItemStack.EMPTY);
                adult.mobInteract(visitor, InteractionHand.MAIN_HAND);
                fields.put("visitor_mounts", visitor.getVehicle() == adult);
                owner.setItemInHand(InteractionHand.MAIN_HAND, ItemStack.EMPTY);
                adult.setYRot(70);
                adult.setXRot(12);
                adult.mobInteract(owner, InteractionHand.MAIN_HAND);
                fields.put("owner_mounts", owner.getVehicle() == adult && adult.getControllingPassenger() == owner);
                fields.put("rider_yaw", owner.getYRot());
                fields.put("rider_pitch", owner.getXRot());
                fields.put("rider_offset", adult.getPassengerRidingPosition(owner).y - adult.getY());
                fields.put("can_push_mounted", adult.isPushable());
                fields.put("daytime_spawns", adult.getType().getCategory().getName());
                owner.stopRiding();
                fields.put("can_push_unmounted", adult.isPushable());
                adult.resetLove();
                adult.setHealth(adult.getMaxHealth());
                var fullOwnerFood = new ItemStack(Items.APPLE, 2);
                owner.setItemInHand(InteractionHand.MAIN_HAND, fullOwnerFood);
                adult.mobInteract(owner, InteractionHand.MAIN_HAND);
                fields.put("full_owner_adult_love", adult.getInLoveTime());
                fields.put("full_owner_food_remaining", fullOwnerFood.getCount());
                adult.resetLove();
                adult.setHealth(20);
                var freshVisitorFood = new ItemStack(Items.APPLE, 2);
                visitor.setItemInHand(InteractionHand.MAIN_HAND, freshVisitorFood);
                adult.mobInteract(visitor, InteractionHand.MAIN_HAND);
                fields.put("injured_visitor_adult_love", adult.getInLoveTime());
                fields.put("injured_visitor_food_remaining", freshVisitorFood.getCount());
                adult.resetLove();
                foal.setHealth(foal.getMaxHealth());
                var babyOwnerFood = new ItemStack(Items.APPLE, 2);
                owner.setItemInHand(InteractionHand.MAIN_HAND, babyOwnerFood);
                foal.mobInteract(owner, InteractionHand.MAIN_HAND);
                fields.put("full_owner_baby_age", foal.getAge());
                fields.put("full_owner_baby_food_remaining", babyOwnerFood.getCount());

                String report = new Gson().toJson(fields);
                source.sendSuccess(() -> Component.literal("naturalist_giraffe_fixture " + report), false);
            } finally {
                adult.discard();
                foal.discard();
                owner.discard();
                visitor.discard();
            }
            return 1;
        }));
    }

    private static List<Float> dimensions(Giraffe animal) {
        return List.of(animal.getBbWidth(), animal.getBbHeight(), animal.getEyeHeight());
    }

    private static Giraffe giraffe(ServerLevel level, double x) {
        var mob = NaturalistEntityTypes.GIRAFFE.get().create(level, EntitySpawnReason.COMMAND);
        mob.snapTo(x, 92, 8.5, 0, 0);
        mob.setNoAi(true);
        mob.setNoGravity(true);
        level.addFreshEntity(mob);
        return mob;
    }

    private static ServerPlayer player(ServerLevel level, String name) {
        var profile = new GameProfile(UUID.randomUUID(), name);
        var player = new ServerPlayer(level.getServer(), level, profile, ClientInformation.createDefault());
        var connection = new Connection(PacketFlow.SERVERBOUND);
        player.connection = new ServerGamePacketListenerImpl(level.getServer(), connection, player,
            CommonListenerCookie.createInitial(profile, false));
        player.snapTo(8.5, 92, 7.5, 0, 0);
        level.addNewPlayer(player);
        player.setGameMode(GameType.SURVIVAL);
        return player;
    }
}
