/* eslint-disable */
// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

declare global {
    interface Window {
        __TAURI_INVOKE__<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
    }
}

// Function avoids 'window not defined' in SSR
const invoke = () => window.__TAURI_INVOKE__;

export function getFullProject(projectId: string) {
    return invoke()<FullProjectContract | null>("get_full_project", { projectId })
}

export function getListProjects() {
    return invoke()<ListProjectContract[]>("get_list_projects")
}

export function createProject(createContract: CreateProjectContract) {
    return invoke()<Project>("create_project", { createContract })
}

export function updateProject(updateContract: UpdateProjectContract) {
    return invoke()<Project>("update_project", { updateContract })
}

export function deleteProject(projectId: string) {
    return invoke()<Project>("delete_project", { projectId })
}

export function createPlaceholder(createContract: CreatePlaceholderContract) {
    return invoke()<Placeholder>("create_placeholder", { createContract })
}

export type Placeholder = { id: string; name: string; variety: string; value: string | null; values: string | null; project_id: string }
export type FullProjectContract = { id: string; name: string; created_at: string; updated_at: string; placeholders: Placeholder[] }
export type Project = { id: string; name: string; created_at: string; updated_at: string }
export type ListProjectContract = { id: string; name: string }
export type UpdateProjectContract = { id: string; name: string }
export type CreatePlaceholderContract = { name: string; variety: string; project_id: string }
export type CreateProjectContract = { name: string }
