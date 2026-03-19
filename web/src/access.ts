export interface AccessState {
  isLogin: boolean;
  currentUser: {
    id: string;
    username: string;
    email: string;
    role: string;
    tenant_id: string;
    is_superuser: boolean;
  } | null;
}

export const initialState: AccessState = {
  isLogin: !!localStorage.getItem('access_token'),
  currentUser: localStorage.getItem('user')
    ? JSON.parse(localStorage.getItem('user')!)
    : null,
};

export default function access(initialState: AccessState) {
  const { currentUser } = initialState || {};
  return {
    canAdmin: currentUser?.is_superuser || currentUser?.role === 'admin',
    canEdit: ['admin', 'editor'].includes(currentUser?.role || ''),
    canView: !!currentUser,
  };
}
