# Simple hook test function to use in gmod
This is a hook tester that you can use to test C -> hook.Call calls

# How to use
```lua
require("hooktest")

-- you can use any function that you want to use
local CurTime = CurTime

for i = 1, 999 do
	hook.Add("HOOK_CALL_TEST", tostring(i), function(arg)
		CurTime()
	end)
end

timer.Simple(1, function()
	print("Started hook test!")
	local time = HOOK_CALL_TEST() -- in seconds
	print("W/e's hook.Call time: " .. time .. "s")
	print("~!")
end)

```
# How to build
	Linux
	cargo build --release --target i686-unknown-linux-gnu

	Windows
	cargo build --release --target i686-pc-windows-msvc

# How to install
You can find the binary file in\
**`target/<i686-W/E-W/E-W/E>/release/(lib)gmsv_hooktest.(so|dll)`**\
rename it to\
**`gmsv_hooktest_linux.dll`** | **`gmsv_hooktest_win32.dll`**\
and place it in\
**`your-gmod-server-directory/garrysmod/lua/bin/`**
