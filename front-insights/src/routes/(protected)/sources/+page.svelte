<script>
    import { auth } from '$lib/stores/auth';
</script>

<div class="p-8">
    <div class="max-w-7xl mx-auto">
        <div class="mb-8">
            <h1 class="text-3xl font-bold text-gray-900">Data Sources</h1>
            <p class="mt-2 text-gray-600">Connect and manage your data sources.</p>
        </div>

        <div class="bg-white rounded-lg shadow p-6">
            <div class="text-center py-12">
                <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4m0 5c0 2.21-3.582 4-8 4s-8-1.79-8-4" />
                </svg>
                <h3 class="mt-2 text-sm font-medium text-gray-900">No data sources connected</h3>
                <p class="mt-1 text-sm text-gray-500">Add a data source to start collecting insights.</p>
            </div>
        </div>
    </div>
</div>
