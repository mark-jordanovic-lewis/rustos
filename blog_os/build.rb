case ARGV[0].to_sym
when :linux then `cp ./templates/system/linux ./src/main.rs`
when :mac then `cp ./templates/system/mac ./src/main.rs`
end

buildcmd = %w[RUST_TARGET_PATH=$(pwd) cargo build --target x86_64-blog_os.json]

exec(buildcmd.join(' '))




# For the case of a std alone binary for running on a host os
#
# buildcmd = case ARGV[0].to_sym
#            when :linux
#              `cp ./templates/system/linux ./src/main.rs`
#              %w[cargo rustc -- -Z pre-link-arg=-nostartfiles]
#            when :mac
#              `cp ./templates/system/mac ./src/main.rs`
#              %w[cargo rustc -- -Z pre-link-arg=-lSystem]
#            end
#
# exec(buildcmd.join(' '))