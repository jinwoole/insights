<script>
    import { auth } from '$lib/stores/auth';
</script>

<div class="p-8">
    <div class="max-w-7xl mx-auto">
        <div class="mb-8">
            <h1 class="text-3xl font-bold text-gray-900">Alerts</h1>
            <p class="mt-2 text-gray-600">Monitor your data and get notified of important changes.</p>
        </div>

        <div class="bg-white rounded-lg shadow p-6">
            <div class="text-center py-12">
                <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
                </svg>
                <h3 class="mt-2 text-sm font-medium text-gray-900">No alerts configured</h3>
                <p class="mt-1 text-sm text-gray-500">Set up alerts to monitor your data.</p>
            </div>
        </div>
    </div>
</div>
