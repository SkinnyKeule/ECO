package com.optimization.eco;

import org.slf4j.Logger;

import com.mojang.logging.LogUtils;

import net.minecraft.network.chat.Component;
import net.minecraft.commands.Commands;

import com.mojang.brigadier.Command;
import net.minecraft.world.entity.Entity;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.phys.AABB;
import net.neoforged.bus.api.IEventBus;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;
import com.mojang.brigadier.arguments.IntegerArgumentType;


@Mod(EntityCollisionOptimization.MODID)
public final class EntityCollisionOptimization {
    public static final String MODID = "eco";
    public static final Logger LOGGER = LogUtils.getLogger();
    public final NativeCollision collision = new NativeCollision();

    public EntityCollisionOptimization(IEventBus modEventBus) {
        NeoForge.EVENT_BUS.addListener(this::registerCommands);
    }

    private void registerCommands(RegisterCommandsEvent event) {
        event.getDispatcher().register(
            Commands.literal(EntityCollisionOptimization.MODID+":buildOctree")
                .executes(context -> {
                    ServerLevel level = context.getSource().getLevel();
                        
                    int count = buildOctree(level);

                    context.getSource().sendSuccess(
                        () -> Component.literal("Built octree with " + count + " entities."), false
                    );
                    return Command.SINGLE_SUCCESS;
                }
            ).requires(s -> s.hasPermission(2)));
        event.getDispatcher().register(
            Commands.literal(EntityCollisionOptimization.MODID+":queryTree")
                .then(
                    Commands.argument("target", IntegerArgumentType.integer())
                        .executes(context -> {
                            int entity_id = IntegerArgumentType.getInteger(context, "target");
                            
                            ServerLevel level = context.getSource().getLevel();
                            Entity entity = level.getEntity(entity_id);

                            if (entity == null) {
                                context.getSource().sendFailure(
                                    Component.literal("No entity found with ID " + entity_id)
                                );
                                return 0;
                            }
                            
                            AABB bounds = entity.getBoundingBox();
                            int[] count = this.collision.query(bounds);

                            String test_count = "";
                            for (int i : count) {
                                test_count += i;
                                test_count += ", ";
                            }
                            final String test = test_count;

                            context.getSource().sendSuccess(
                                () -> Component.literal("Found [" + test + "] overlapping entities for " + entity.getName().getString() + " with id " + entity_id), false
                            );
                            
                            return Command.SINGLE_SUCCESS;
                        })
                ).requires(s -> s.hasPermission(2)));
        event.getDispatcher().register(
            Commands.literal(EntityCollisionOptimization.MODID+":removeFromTree")
                .then(
                    Commands.argument("target", IntegerArgumentType.integer())
                        .executes(context -> {
                            int entity_id = IntegerArgumentType.getInteger(context, "target");

                            int count = this.collision.delete(entity_id);

                            context.getSource().sendSuccess(
                                () -> Component.literal("Removed entity " + entity_id + ". " + count + " entities remaining in the tree"), false
                            );
                            
                            return Command.SINGLE_SUCCESS;
                        })
                ).requires(s -> s.hasPermission(2)));
    }

    private int buildOctree(ServerLevel level) {
        int count = 0;

        for (Entity entity : level.getAllEntities()) {
            AABB box = entity.getBoundingBox();

            this.collision.upsert(
                entity.getId(),
                box
            );

            LOGGER.info("Entity: {} ID: {}", entity.getName().getString(), entity.getId());
            count++;
        }

        LOGGER.info("Built octree with {} entities in {}", count,
                level.dimension().location());

        return count;
    }
}
