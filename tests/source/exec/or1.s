	.text
	.globl main
main:
	movq $65, %rdi
	movq $1, %rcx
	testq %rcx, %rcx
	jnz L3988
	movq $1, %rcx
	testq %rcx, %rcx
	jz L3987
L3988:
	movq $1, %rcx
L3987:
	addq %rcx, %rdi
	call putchar
	addq $0, %rsp
	movq $65, %rdi
	movq $0, %rcx
	testq %rcx, %rcx
	jnz L3980
	movq $2, %rcx
	testq %rcx, %rcx
	jz L3979
L3980:
	movq $1, %rcx
L3979:
	addq %rcx, %rdi
	call putchar
	addq $0, %rsp
	movq $65, %rdi
	movq $1, %rdx
	testq %rdx, %rdx
	jnz L3972
	movq $0, %rdx
	testq %rdx, %rdx
	jz L3971
L3972:
	movq $1, %rdx
L3971:
	addq %rdx, %rdi
	call putchar
	addq $0, %rsp
	movq $65, %rdi
	movq $0, %r8
	testq %r8, %r8
	jnz L3964
	movq $0, %r8
	testq %r8, %r8
	jz L3963
L3964:
	movq $1, %r8
L3963:
	addq %r8, %rdi
	call putchar
	addq $0, %rsp
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	ret
	jmp L3963
	jmp L3964
	jmp L3971
	jmp L3972
	jmp L3979
	jmp L3980
	jmp L3987
	jmp L3988
	.data

