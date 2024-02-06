	.text
	.globl main
main:
	movq $65, %rdi
	movq $1, %r8
	testq %r8, %r8
	jnz L4087
	movq $1, %r8
	testq %r8, %r8
	jz L4086
L4087:
	movq $1, %r8
L4086:
	addq %r8, %rdi
	call putchar
	movq $65, %rdi
	movq $0, %rax
	testq %rax, %rax
	jnz L4079
	movq $2, %rax
	testq %rax, %rax
	jz L4078
L4079:
	movq $1, %rax
L4078:
	addq %rax, %rdi
	call putchar
	movq $65, %rdi
	movq $1, %rcx
	testq %rcx, %rcx
	jnz L4071
	movq $0, %rcx
	testq %rcx, %rcx
	jz L4070
L4071:
	movq $1, %rcx
L4070:
	addq %rcx, %rdi
	call putchar
	movq $65, %rdi
	movq $0, %rcx
	testq %rcx, %rcx
	jnz L4063
	movq $0, %rcx
	testq %rcx, %rcx
	jz L4062
L4063:
	movq $1, %rcx
L4062:
	addq %rcx, %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	ret
	jmp L4062
	jmp L4063
	jmp L4070
	jmp L4071
	jmp L4078
	jmp L4079
	jmp L4086
	jmp L4087
	.data

