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

@Mod(EntityCollisionOptimization.MODID)
public final class EntityCollisionOptimization {
    public static final String MODID = "eco";
    public static final Logger LOGGER = LogUtils.getLogger();

    public EntityCollisionOptimization(IEventBus modEventBus) {
        NeoForge.EVENT_BUS.addListener(this::registerCommands);
    }

    private void registerCommands(RegisterCommandsEvent event) {
        event.getDispatcher().register(
                Commands.literal("buildoctree")
                        .requires(source -> source.hasPermission(2))
                        .executes(context -> {
                            ServerLevel level = context.getSource().getLevel();

                            int count = buildOctree(level);

                            context.getSource().sendSuccess(
                                    () -> Component.literal(
                                            "Built octree with " + count + " entities."),
                                    false);

                            return Command.SINGLE_SUCCESS;
                        }));
    }

    private int buildOctree(ServerLevel level) {
        NativeCollision.clearTree();

        int count = 0;

        for (Entity entity : level.getAllEntities()) {
            AABB box = entity.getBoundingBox();

            NativeCollision.insertEntity(
                    entity.getId(),
                    box.minX, box.minY, box.minZ,
                    box.maxX, box.maxY, box.maxZ);

            count++;
        }

        LOGGER.info("Built octree with {} entities in {}", count,
                level.dimension().location());

        return count;
    }
}
