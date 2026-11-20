package naturalist.fixture;
import com.google.gson.Gson;
import java.util.ArrayList;
import java.util.List;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.network.chat.Component;
import net.minecraft.world.attribute.EnvironmentAttributes;
import net.minecraft.world.entity.MobCategory;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;
@Mod("naturalist_fixture")
public final class BirdDensityTableProbe {
 public BirdDensityTableProbe() { NeoForge.EVENT_BUS.addListener(this::register); }
 private void register(RegisterCommandsEvent e) {
  e.getDispatcher().register(Commands.literal("bird_density_table").executes(ctx -> {
   var level=ctx.getSource().getLevel();
   var modifiers=level.registryAccess().lookupOrThrow(net.neoforged.neoforge.registries.NeoForgeRegistries.Keys.BIOME_MODIFIERS).listElements().map(v -> v.key().identifier().toString()).toList();
   ctx.getSource().sendSuccess(()->Component.literal("bird_density_modifiers "+new Gson().toJson(modifiers)),false);
   for (var p : List.of(new BlockPos(4236,88,3964), new BlockPos(4237,88,3964), new BlockPos(4232,81,3967))) {
    var bird=com.crispytwig.naturalist.registry.NaturalistEntityTypes.BIRD.get();
    boolean b=net.minecraft.world.entity.SpawnPlacements.isSpawnPositionOk(bird,level,p);
    boolean parrot=net.minecraft.world.entity.SpawnPlacements.isSpawnPositionOk(net.minecraft.world.entity.EntityTypes.PARROT,level,p);
    boolean rule=com.crispytwig.naturalist.world.entity.animal.bird.Bird.checkBirdSpawnRules(bird,level,net.minecraft.world.entity.EntitySpawnReason.CHUNK_GENERATION,p,level.getRandom());
    var below=level.getBlockState(p.below()); var at=level.getBlockState(p); var above=level.getBlockState(p.above());
    boolean supportBird=below.isValidSpawn(level,p.below(),bird);
    boolean supportParrot=below.isValidSpawn(level,p.below(),net.minecraft.world.entity.EntityTypes.PARROT);
    ctx.getSource().sendSuccess(()->Component.literal("bird_placement "+p+" bird="+b+" parrot="+parrot+" bird_rule="+rule+" supportBird="+supportBird+" supportParrot="+supportParrot+" blocks="+below+","+at+","+above),false);
   }
   var biome=level.getBiome(new BlockPos(4224,319,3952));
   var attribute=EnvironmentAttributes.NATURAL_MOB_SPAWNS;
   var entry=biome.value().getModifiedEnvironmentAttributes().get(attribute);
   var table=entry==null?attribute.defaultValue():entry.applyModifier(attribute.defaultValue());
   var rows=new ArrayList<List<Object>>();
   for (var weighted:table.getMobsToSpawn(MobCategory.CREATURE).unwrap()) {
    var spawn=weighted.value();rows.add(List.of(BuiltInRegistries.ENTITY_TYPE.getKey(spawn.type()).toString(),weighted.weight(),spawn.count().minInclusive(),spawn.count().maxInclusive()));
   }
   String out=new Gson().toJson(List.of(biome.unwrapKey().orElseThrow().identifier().toString(),rows));
   ctx.getSource().sendSuccess(()->Component.literal("bird_density_table "+out),false);return 1;
  }));
 }
}
