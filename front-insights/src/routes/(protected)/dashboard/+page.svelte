<script>
    import { auth } from '$lib/stores/auth';
</script>

<div class="p-8">
    <div class="max-w-7xl mx-auto">
        <div class="mb-8">
            <h1 class="text-3xl font-bold text-gray-900">Welcome, {$auth.userInfo?.username || 'User'}!</h1>
            <p class="mt-2 text-gray-600">Here's your personal analytics dashboard.</p>
        </div>

        <!-- Dashboard Grid -->
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            <!-- Analytics Card 1 -->
            <div class="bg-white rounded-lg shadow p-6">
                <div class="flex items-center justify-between mb-4">
                    <h3 class="text-lg font-medium text-gray-900">Data Overview</h3>
                    <span class="bg-blue-100 text-blue-800 text-xs font-medium px-2.5 py-0.5 rounded">Today</span>
                </div>
                <div class="text-3xl font-bold text-gray-900 mb-2">0</div>
                <p class="text-sm text-gray-500">Total data points collected</p>
            </div>

            <!-- Analytics Card 2 -->
            <div class="bg-white rounded-lg shadow p-6">
                <div class="flex items-center justify-between mb-4">
                    <h3 class="text-lg font-medium text-gray-900">Active Sources</h3>
                    <span class="bg-green-100 text-green-800 text-xs font-medium px-2.5 py-0.5 rounded">Live</span>
                </div>
                <div class="text-3xl font-bold text-gray-900 mb-2">0</div>
                <p class="text-sm text-gray-500">Connected data sources</p>
            </div>

            <!-- Analytics Card 3 -->
            <div class="bg-white rounded-lg shadow p-6">
                <div class="flex items-center justify-between mb-4">
                    <h3 class="text-lg font-medium text-gray-900">Alerts</h3>
                    <span class="bg-yellow-100 text-yellow-800 text-xs font-medium px-2.5 py-0.5 rounded">Pending</span>
                </div>
                <div class="text-3xl font-bold text-gray-900 mb-2">0</div>
                <p class="text-sm text-gray-500">Active alerts</p>
            </div>
        </div>

        <!-- Recent Activity -->
        <div class="mt-8">
            <div class="bg-white rounded-lg shadow">
                <div class="p-6">
                    <h3 class="text-lg font-medium text-gray-900 mb-4">Recent Activity</h3>
                    <div class="text-center py-8 text-gray-500">
                        No recent activity to display.
                        <br>
                        Start collecting data to see your insights here!
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>
