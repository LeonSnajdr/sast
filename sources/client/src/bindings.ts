/* eslint-disable */
// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

declare global {
    interface Window {
        __TAURI_INVOKE__<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
    }
}

// Function avoids 'window not defined' in SSR
const invoke = () => window.__TAURI_INVOKE__;

export function getProjects() {
    return invoke()<Project[]>("get_projects")
}

export function createProject(createData: CreateProjectContract) {
    return invoke()<Project>("create_project", { createData })
}

export type Project = { id: string; name: string; createdAt: string; updatedAt: string }
export type CreateProjectContract = { name: string }
