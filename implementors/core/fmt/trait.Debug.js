(function() {var implementors = {};
implementors['libc'] = [];implementors['log'] = ["impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html' title='core::fmt::Debug'>Debug</a> for <a class='enum' href='log/enum.LogLevel.html' title='log::LogLevel'>LogLevel</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html' title='core::fmt::Debug'>Debug</a> for <a class='enum' href='log/enum.LogLevelFilter.html' title='log::LogLevelFilter'>LogLevelFilter</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html' title='core::fmt::Debug'>Debug</a> for <a class='struct' href='log/struct.LogLocation.html' title='log::LogLocation'>LogLocation</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html' title='core::fmt::Debug'>Debug</a> for <a class='struct' href='log/struct.MaxLogLevelFilter.html' title='log::MaxLogLevelFilter'>MaxLogLevelFilter</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html' title='core::fmt::Debug'>Debug</a> for <a class='struct' href='log/struct.SetLoggerError.html' title='log::SetLoggerError'>SetLoggerError</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html' title='core::fmt::Debug'>Debug</a> for <a class='struct' href='log/struct.ShutdownLoggerError.html' title='log::ShutdownLoggerError'>ShutdownLoggerError</a>",];implementors['time'] = ["impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html' title='core::fmt::Debug'>Debug</a> for <a class='struct' href='time/struct.Duration.html' title='time::Duration'>Duration</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html' title='core::fmt::Debug'>Debug</a> for <a class='struct' href='time/struct.Timespec.html' title='time::Timespec'>Timespec</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html' title='core::fmt::Debug'>Debug</a> for <a class='struct' href='time/struct.SteadyTime.html' title='time::SteadyTime'>SteadyTime</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html' title='core::fmt::Debug'>Debug</a> for <a class='struct' href='time/struct.Tm.html' title='time::Tm'>Tm</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html' title='core::fmt::Debug'>Debug</a> for <a class='enum' href='time/enum.ParseError.html' title='time::ParseError'>ParseError</a>","impl&lt;'a&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html' title='core::fmt::Debug'>Debug</a> for <a class='struct' href='time/struct.TmFmt.html' title='time::TmFmt'>TmFmt</a>&lt;'a&gt;",];implementors['syncbox'] = ["impl&lt;T: <a class='trait' href='https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html' title='core::fmt::Debug'>Debug</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html' title='core::fmt::Debug'>Debug</a> for <a class='struct' href='syncbox/struct.Delay.html' title='syncbox::Delay'>Delay</a>&lt;T&gt;",];implementors['eventual'] = ["impl&lt;T: <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Send.html' title='core::marker::Send'>Send</a> + 'static, E: <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Send.html' title='core::marker::Send'>Send</a> + 'static&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html' title='core::fmt::Debug'>Debug</a> for <a class='struct' href='eventual/struct.Future.html' title='eventual::Future'>Future</a>&lt;T, E&gt;","impl&lt;T: <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Send.html' title='core::marker::Send'>Send</a> + 'static, E: <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Send.html' title='core::marker::Send'>Send</a> + 'static&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html' title='core::fmt::Debug'>Debug</a> for <a class='struct' href='eventual/struct.Complete.html' title='eventual::Complete'>Complete</a>&lt;T, E&gt;","impl&lt;T: <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Send.html' title='core::marker::Send'>Send</a> + 'static, E: <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Send.html' title='core::marker::Send'>Send</a> + 'static&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html' title='core::fmt::Debug'>Debug</a> for <a class='struct' href='eventual/struct.Stream.html' title='eventual::Stream'>Stream</a>&lt;T, E&gt;","impl&lt;T: <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Send.html' title='core::marker::Send'>Send</a> + 'static, E: <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Send.html' title='core::marker::Send'>Send</a> + 'static&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html' title='core::fmt::Debug'>Debug</a> for <a class='struct' href='eventual/struct.Sender.html' title='eventual::Sender'>Sender</a>&lt;T, E&gt;","impl&lt;E: <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Send.html' title='core::marker::Send'>Send</a> + 'static + <a class='trait' href='https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html' title='core::fmt::Debug'>Debug</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html' title='core::fmt::Debug'>Debug</a> for <a class='enum' href='eventual/enum.AsyncError.html' title='eventual::AsyncError'>AsyncError</a>&lt;E&gt;",];implementors['robots'] = ["impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html' title='core::fmt::Debug'>Debug</a> for <a class='enum' href='robots/actors/actor_ref/enum.ActorPath.html' title='robots::actors::actor_ref::ActorPath'>ActorPath</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html' title='core::fmt::Debug'>Debug</a> for <a class='struct' href='robots/actors/actor_ref/struct.ConnectionInfo.html' title='robots::actors::actor_ref::ConnectionInfo'>ConnectionInfo</a>",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()