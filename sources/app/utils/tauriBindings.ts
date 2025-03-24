
// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

/** user-defined commands **/


export const commands = {
async settingInitialize(initializeContract: SettingInitializeContract) : Promise<Result<SettingContract, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("setting_initialize", { initializeContract }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async settingGetDefault() : Promise<Result<SettingContract | null, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("setting_get_default") };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async settingUpdateOne(updateContract: SettingUpdateContract) : Promise<Result<null, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("setting_update_one", { updateContract }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async projectCreate(projectCreateContract: ProjectCreateContract) : Promise<Result<ProjectContract, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("project_create", { projectCreateContract }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async projectGetAll() : Promise<Result<ProjectContract[], Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("project_get_all") };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async projectGetLastOpened() : Promise<Result<ProjectContract | null, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("project_get_last_opened") };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async projectOpen(id: string) : Promise<Result<ProjectContract, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("project_open", { id }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async terminalSpawn(spawnContract: TerminalSpawnContract) : Promise<Result<string, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("terminal_spawn", { spawnContract }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async terminalWrite(id: string, data: string) : Promise<Result<null, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("terminal_write", { id, data }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async terminalGetReadHistory(id: string) : Promise<Result<string, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("terminal_get_read_history", { id }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async terminalGetManyInfo(filter: TerminalFilterContract) : Promise<Result<TerminalInfoContract[], Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("terminal_get_many_info", { filter }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async terminalResize(id: string, resizeContract: TerminalResizeContract) : Promise<Result<null, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("terminal_resize", { id, resizeContract }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async terminalKill(id: string) : Promise<Result<null, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("terminal_kill", { id }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async terminalDelete(id: string) : Promise<Result<null, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("terminal_delete", { id }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async placeholderCreate(placeholderCreateContract: PlaceholderCreateContract) : Promise<Result<PlaceholderContract, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("placeholder_create", { placeholderCreateContract }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async placeholderGetMany(projectId: string) : Promise<Result<PlaceholderContract[], Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("placeholder_get_many", { projectId }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async placeholderGetOne(id: string) : Promise<Result<PlaceholderContract, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("placeholder_get_one", { id }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async placeholderUpdateOne(placeholderUpdateContract: PlaceholderUpdateContract) : Promise<Result<null, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("placeholder_update_one", { placeholderUpdateContract }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async placeholderDeleteOne(id: string) : Promise<Result<null, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("placeholder_delete_one", { id }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async taskCreate(taskCreateContract: TaskCreateContract) : Promise<Result<TaskContract, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("task_create", { taskCreateContract }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async taskGetOne(id: string) : Promise<Result<TaskContract, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("task_get_one", { id }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async taskGetManyInfo(projectId: string) : Promise<Result<TaskInfoContract[], Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("task_get_many_info", { projectId }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async taskUpdateOne(taskUpdateContract: TaskUpdateContract) : Promise<Result<TaskContract, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("task_update_one", { taskUpdateContract }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async taskDeleteOne(id: string) : Promise<Result<null, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("task_delete_one", { id }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async taskStartOne(projectId: string, taskId: string) : Promise<Result<null, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("task_start_one", { projectId, taskId }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async taskRestartOne(projectId: string, taskId: string) : Promise<Result<null, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("task_restart_one", { projectId, taskId }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async taskStopOne(taskId: string) : Promise<Result<null, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("task_stop_one", { taskId }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async taskSetCreate(taskSetCreateContract: TaskSetCreateContract) : Promise<Result<string, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("task_set_create", { taskSetCreateContract }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async taskSetGetOne(id: string) : Promise<Result<TaskSetContract, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("task_set_get_one", { id }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async taskSetGetManyInfo(projectId: string) : Promise<Result<TaskSetInfoContract[], Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("task_set_get_many_info", { projectId }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async taskSetUpdateOne(taskSetUpdateContract: TaskSetUpdateContract) : Promise<Result<null, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("task_set_update_one", { taskSetUpdateContract }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async taskSetDeleteOne(id: string) : Promise<Result<null, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("task_set_delete_one", { id }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async taskSetStartOne(projectId: string, taskSetId: string) : Promise<Result<null, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("task_set_start_one", { projectId, taskSetId }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async taskSetRestartOne(projectId: string, taskSetId: string) : Promise<Result<null, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("task_set_restart_one", { projectId, taskSetId }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async taskSetStopOne(taskSetId: string) : Promise<Result<null, Error>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("task_set_stop_one", { taskSetId }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
}
}

/** user-defined events **/


export const events = __makeEvents__<{
terminalCreatedEvent: TerminalCreatedEvent,
terminalDeletedEvent: TerminalDeletedEvent,
terminalShellKilledEvent: TerminalShellKilledEvent,
terminalShellReadEvent: TerminalShellReadEvent,
terminalShellSpawnedEvent: TerminalShellSpawnedEvent
}>({
terminalCreatedEvent: "terminal-created-event",
terminalDeletedEvent: "terminal-deleted-event",
terminalShellKilledEvent: "terminal-shell-killed-event",
terminalShellReadEvent: "terminal-shell-read-event",
terminalShellSpawnedEvent: "terminal-shell-spawned-event"
})

/** user-defined constants **/



/** user-defined types **/

export type Error = { Db: string } | "AlreadyExists" | "NotExists" | "InvalidStatus" | "Failed"
export type PlaceholderContract = { id: string; projectId: string; name: string; value: string; visibility: PlaceholderVisibility; kind: PlaceholderKind; source: PlaceholderSource; dateCreated: string; dateLastUpdated: string }
export type PlaceholderCreateContract = { projectId: string; name: string; value: string; visibility: PlaceholderVisibility; kind: PlaceholderKind; source: PlaceholderSource }
export type PlaceholderInsertTileContract = { kind: PlaceholderInsertTileKind; textValue: string | null; placeholderId: string | null; placeholderName: string | null; placeholderVisibility: PlaceholderVisibility | null }
export type PlaceholderInsertTileKind = "Text" | "Placeholder"
export type PlaceholderKind = "Text" | "Select"
export type PlaceholderSource = "Static"
export type PlaceholderUpdateContract = { id: string; projectId: string; name: string; value: string; visibility: PlaceholderVisibility; kind: PlaceholderKind; source: PlaceholderSource }
export type PlaceholderVisibility = "Global" | "Project"
export type ProjectContract = { id: string; name: string; dateCreated: string; dateLastOpened: string }
export type ProjectCreateContract = { name: string }
export type SettingContract = { id: string; metaDateUpdated: string; presentationLanguage: string; presentationTheme: string; behaviorOpenWelcome: boolean }
export type SettingInitializeContract = { presentationLanguage: string; presentationTheme: string }
export type SettingUpdateContract = { id: string; presentationLanguage: string; presentationTheme: string; behaviorOpenWelcome: boolean }
export type TaskContract = { id: string; projectId: string; name: string; tabName: string | null; noExit: boolean; forceKill: boolean; historyPersistence: TerminalHistoryPersistence; commandTiles: PlaceholderInsertTileContract[]; workingDirTiles: PlaceholderInsertTileContract[]; dateCreated: string; dateLastUpdated: string }
export type TaskCreateContract = { projectId: string; name: string; tabName: string | null; noExit: boolean; forceKill: boolean; historyPersistence: TerminalHistoryPersistence; commandTiles: PlaceholderInsertTileContract[]; workingDirTiles: PlaceholderInsertTileContract[] }
export type TaskInfoContract = { id: string; projectId: string; name: string; dateCreated: string; dateLastUpdated: string }
export type TaskSetContract = { id: string; projectId: string; name: string; dateCreated: string; dateLastUpdated: string; tasks: TaskSetTaskInfoContract[] }
export type TaskSetCreateContract = { projectId: string; name: string }
export type TaskSetInfoContract = { id: string; projectId: string; name: string; dateCreated: string; dateLastUpdated: string; taskIds: string[] }
export type TaskSetTaskInfoContract = { taskId: string; taskName: string; taskDateCreated: string; taskDateLastUpdated: string; blocking: boolean }
export type TaskSetUpdateContract = { id: string; name: string; tasks: TaskSetTaskInfoContract[] }
export type TaskUpdateContract = { id: string; name: string; tabName: string | null; noExit: boolean; forceKill: boolean; historyPersistence: TerminalHistoryPersistence; commandTiles: PlaceholderInsertTileContract[]; workingDirTiles: PlaceholderInsertTileContract[] }
export type TerminalCreatedEvent = string
export type TerminalDeletedEvent = string
export type TerminalFilterContract = { id: string | null; projectId: string | null; taskId: string | null }
export type TerminalHistoryPersistence = "Always" | "Never" | "OnError" | "OnSuccess"
export type TerminalInfoContract = { id: string; projectId: string; task: TaskInfoContract | null; name: string; shellStatus: TerminalShellStatus }
export type TerminalResizeContract = { cols: number; rows: number }
export type TerminalShellKilledEvent = string
export type TerminalShellReadEvent = TerminalShellReadEventData
export type TerminalShellReadEventData = { id: string; data: string }
export type TerminalShellSpawnedEvent = string
export type TerminalShellStatus = "Creating" | "Running" | "Restarting" | "Killing" | "Killed"
export type TerminalSpawnContract = { projectId: string; taskId: string | null; name: string | null; workingDir: string | null; command: string | null; noExit: boolean; forceKill: boolean; historyPersistence: TerminalHistoryPersistence }

/** tauri-specta globals **/

import {
	invoke as TAURI_INVOKE,
	Channel as TAURI_CHANNEL,
} from "@tauri-apps/api/core";
import * as TAURI_API_EVENT from "@tauri-apps/api/event";
import { type WebviewWindow as __WebviewWindow__ } from "@tauri-apps/api/webviewWindow";

type __EventObj__<T> = {
	listen: (
		cb: TAURI_API_EVENT.EventCallback<T>,
	) => ReturnType<typeof TAURI_API_EVENT.listen<T>>;
	once: (
		cb: TAURI_API_EVENT.EventCallback<T>,
	) => ReturnType<typeof TAURI_API_EVENT.once<T>>;
	emit: null extends T
		? (payload?: T) => ReturnType<typeof TAURI_API_EVENT.emit>
		: (payload: T) => ReturnType<typeof TAURI_API_EVENT.emit>;
};

export type Result<T, E> =
	| { status: "ok"; data: T }
	| { status: "error"; error: E };

function __makeEvents__<T extends Record<string, any>>(
	mappings: Record<keyof T, string>,
) {
	return new Proxy(
		{} as unknown as {
			[K in keyof T]: __EventObj__<T[K]> & {
				(handle: __WebviewWindow__): __EventObj__<T[K]>;
			};
		},
		{
			get: (_, event) => {
				const name = mappings[event as keyof T];

				return new Proxy((() => {}) as any, {
					apply: (_, __, [window]: [__WebviewWindow__]) => ({
						listen: (arg: any) => window.listen(name, arg),
						once: (arg: any) => window.once(name, arg),
						emit: (arg: any) => window.emit(name, arg),
					}),
					get: (_, command: keyof __EventObj__<any>) => {
						switch (command) {
							case "listen":
								return (arg: any) => TAURI_API_EVENT.listen(name, arg);
							case "once":
								return (arg: any) => TAURI_API_EVENT.once(name, arg);
							case "emit":
								return (arg: any) => TAURI_API_EVENT.emit(name, arg);
						}
					},
				});
			},
		},
	);
}
