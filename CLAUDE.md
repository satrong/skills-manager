# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Tauri v2 desktop application with a Vue 3 frontend. Tauri allows building desktop apps using web technologies with a Rust backend for native functionality.

## Development Commands

- `pnpm dev` - Start development server (Vite dev server on port 1420, required for Tauri dev)
- `pnpm build` - Build for production (runs TypeScript check via `vue-tsc --noEmit` then Vite build)
- `pnpm tauri dev` - Run Tauri in development mode (automatically runs `beforeDevCommand: pnpm dev`)
- `pnpm tauri build` - Build the desktop application
- `vue-tsc --noEmit` - Type check Vue components and TypeScript files

## Architecture

**Frontend (Vue 3 + TypeScript)**
- Uses Vue 3 Composition API with `<script setup>` syntax
- Entry point: `src/main.ts` → `src/App.vue`
- Vite dev server runs on fixed port 1420 (required by Tauri)
- TypeScript strict mode enabled with `noUnusedLocals` and `noUnusedParameters`

**Backend (Rust via Tauri)**
- Entry point: `src-tauri/src/main.rs` calls `skills_manager_lib::run()`
- Core logic: `src-tauri/src/lib.rs` - defines Tauri commands and initializes the app
- Tauri commands are exposed via `#[tauri::command]` macro and registered in `invoke_handler`
- Frontend calls Rust commands via `invoke()` from `@tauri-apps/api/core`

**Communication Pattern**
- Frontend → Backend: Use `invoke("command_name", { params })` from `@tauri-apps/api/core`
- Backend commands must be registered in `tauri::generate_handler![...]` in `lib.rs`

## Configuration

- `src-tauri/tauri.conf.json` - Main Tauri configuration (window size, build commands, app identifier)
- `vite.config.ts` - Vite is configured to ignore watching `src-tauri` directory
- App identifier: `com.satrong.skills-manager`
