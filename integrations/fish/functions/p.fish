function p_cd_completions
    project-manager --projects-root $PROJECT_HOME list-projects
    #"Switch to project" | awk '{sub(rpl, "", $1); print $1}' rpl="$PROJECT_HOME/" | awk '{sub("/(?:.(?!/))*$", "", $0); print $0}'
end

function p_completions
    set -l commands cd help new clone

    complete -c p -ef
    complete -c p -n "not __fish_seen_subcommand_from $commands"  -kxa "help" -d "Display help"
    complete -c p -n "not __fish_seen_subcommand_from $commands"  -kxa "new" -d "Create new project"
    complete -c p -n "not __fish_seen_subcommand_from $commands"  -kxa "clone" -d "Clone project with git"
    complete -c p -n "not __fish_seen_subcommand_from $commands"  -kxa "cd" -d "Change to project dir"

# completions for project-manager cd
    complete -c p -n "__fish_seen_subcommand_from cd" \
        -d "Switch project" \
        -xa "(p_cd_completions | string split \n)"

# completions for project-manager new
    complete -c p -n "__fish_seen_subcommand_from new" -f

# completions for project-manager clone
    complete -c p -n "__fish_seen_subcommand_from clone" -f
    complete -c p -n "__fish_seen_subcommand_from clone" -s p -l project-name -r -f
    complete -c p -n "__fish_seen_subcommand_from clone" -s d -l parent-dir -r --force-files

# completions for project-manager help
    complete -c p -n "__fish_seen_subcommand_from help" \
        -xa (echo $commands) \
        -d "Display help"
end

p_completions

function p
    if test \( -n $argv[1] \) -a \( $argv[1] = "cd" \)
        cd (project-manager --projects-root $PROJECT_HOME cd $argv[2])
    else
        project-manager --projects-root $PROJECT_HOME $argv
    end
end
