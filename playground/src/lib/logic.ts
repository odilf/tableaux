import { Classical, Modal, NormalModal } from '$rust';

export const logics = ['classical', 'modal', 'normalModal'] as const;
export type LogicKind = (typeof logics)[number];
export const asLogicKind = (value: string): LogicKind => {
	if (logics.includes(value as LogicKind)) {
		return value as LogicKind;
	} else {
		throw new Error('Invalid logic');
	}
};

export const classObject = {
	classical: Classical,
	modal: Modal,
	normalModal: NormalModal
} satisfies Record<LogicKind, unknown>;

export const displayName = {
	classical: 'Classical',
	modal: 'Basic modal',
	normalModal: 'Normal modal'
} satisfies Record<LogicKind, string>;

export const chapterLogics: Record<string, LogicKind> = {
	1: 'classical',
	2: 'modal',
	3: 'normalModal'
};
