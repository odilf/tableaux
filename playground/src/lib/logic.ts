import { Classical, Modal, NormalModal } from '$rust';

export const logics = ['classical', 'modal', 'normalModal'] as const;
export type LogicKind = (typeof logics)[number];

export const classObject = {
	classical: Classical,
	modal: Modal,
	normalModal: NormalModal
} satisfies Record<LogicKind, unknown>;

export const displayName = {
	classical: 'Classical',
	modal: 'Basic modal',
	normalModal: 'Normal Modal'
} satisfies Record<LogicKind, string>;

export const chapterLogics = ['classical', 'modal', 'normalModal'] as const;
